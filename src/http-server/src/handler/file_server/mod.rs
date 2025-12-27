mod service;
mod utils;

use std::fs::read_dir;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, request::Parts, HeaderValue, Method, Response, StatusCode, Uri};
use http_body_util::{BodyExt, Full};
use multer::Multipart;
use percent_encoding::{percent_decode_str, utf8_percent_encode};
use tokio::io::AsyncWriteExt;

use crate::server::{HttpRequest, HttpResponse};

use self::service::FileServer as FileServerService;

pub struct FileServer {
    file_service: FileServerService,
    path: PathBuf,
}

impl FileServer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_service: FileServerService::new(path.clone()),
            path,
        }
    }

    pub async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        let (parts, body) = req.into_parts();
        let body = body.collect().await.unwrap().to_bytes();

        if parts.uri.path().starts_with("/api/v1") {
            return self.handle_api(parts, body).await;
        }

        let path = parts.uri.path();
        let path = path.strip_prefix('/').unwrap_or(path);

        if let Some(file) = FileExplorerAssets::get(path) {
            let content_type = mime_guess::from_path(path).first_or_octet_stream();
            let content_type = HeaderValue::from_str(content_type.as_ref()).unwrap();
            let body = Full::new(Bytes::from(file.data.to_vec()));
            let mut response = Response::new(body);
            let mut headers = response.headers().clone();

            headers.append(CONTENT_TYPE, content_type);
            *response.headers_mut() = headers;

            return Ok(response);
        }

        let index = FileExplorerAssets::get("index.html").unwrap();
        let body = Full::new(Bytes::from(index.data.to_vec()));
        let mut response = Response::new(body);
        let mut headers = response.headers().clone();

        headers.append(CONTENT_TYPE, "text/html".try_into().unwrap());
        *response.headers_mut() = headers;

        Ok(response)
    }

    async fn handle_api(&self, parts: Parts, body: Bytes) -> Result<HttpResponse> {
        let path = Self::parse_req_uri(parts.uri.clone()).unwrap();

        match parts.method {
            Method::GET => match self.file_explorer.peek(path).await {
                Ok(entry) => match entry {
                    Entry::Directory(dir) => {
                        let directory_index =
                            self.marshall_directory_index(dir.path()).await.unwrap();
                        let json = serde_json::to_string(&directory_index).unwrap();
                        let body = Full::new(Bytes::from(json));
                        let mut response = Response::new(body);
                        let mut headers = response.headers().clone();

                        headers.append(CONTENT_TYPE, "application/json".try_into().unwrap());
                        *response.headers_mut() = headers;

                        Ok(response)
                    }
                    Entry::File(mut file) => {
                        let body = Full::new(Bytes::from(file.bytes().await.unwrap()));
                        let mut response = Response::new(body);
                        let mut headers = response.headers().clone();

                        headers.append(CONTENT_TYPE, file.mime().to_string().try_into().unwrap());
                        *response.headers_mut() = headers;

                        Ok(response)
                    }
                },
                Err(err) => {
                    let message = format!("Failed to resolve path: {err}");
                    Ok(Response::new(Full::new(Bytes::from(message))))
                }
            },
            Method::POST => {
                self.handle_file_upload(parts, body).await?;
                Ok(Response::new(Full::new(Bytes::from(
                    "POST method is not supported",
                ))))
            }
            _ => Ok(Response::new(Full::new(Bytes::from("Unsupported method")))),
        }
    }

    async fn handle_file_upload(&self, parts: Parts, body: Bytes) -> Result<HttpResponse> {
        // Extract the `multipart/form-data` boundary from the headers.
        let boundary = parts
            .headers
            .get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .and_then(|ct| multer::parse_boundary(ct).ok());

        // Send `BAD_REQUEST` status if the content-type is not multipart/form-data.
        if boundary.is_none() {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::from("BAD REQUEST"))
                .unwrap());
        }

        // Process the multipart e.g. you can store them in files.
        if let Err(err) = self.process_multipart(body, boundary.unwrap()).await {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("INTERNAL SERVER ERROR: {err}")))
                .unwrap());
        }

        Ok(Response::new(Full::from("Success")))
    }

    async fn process_multipart(&self, bytes: Bytes, boundary: String) -> multer::Result<()> {
        let cursor = std::io::Cursor::new(bytes);
        let bytes_stream = tokio_util::io::ReaderStream::new(cursor);
        let mut multipart = Multipart::new(bytes_stream, boundary);

        // Iterate over the fields, `next_field` method will return the next field if
        // available.
        while let Some(mut field) = multipart.next_field().await? {
            // Get the field name.
            let name = field.name();

            // Get the field's filename if provided in "Content-Disposition" header.
            let file_name = field.file_name().to_owned().unwrap_or("default.png");

            // Get the "Content-Type" header as `mime::Mime` type.
            let content_type = field.content_type();

            let mut file = tokio::fs::File::create(file_name).await.unwrap();

            println!(
                "\n\nName: {name:?}, FileName: {file_name:?}, Content-Type: {content_type:?}\n\n"
            );

            // Process the field data chunks e.g. store them in a file.
            let mut field_bytes_len = 0;
            while let Some(field_chunk) = field.chunk().await? {
                // Do something with field chunk.
                field_bytes_len += field_chunk.len();
                file.write_all(&field_chunk).await.unwrap();
            }

            println!("Field Bytes Length: {field_bytes_len:?}");
        }

        Ok(())
    }

    fn parse_req_uri(uri: Uri) -> Result<PathBuf> {
        let parts: Vec<&str> = uri.path().split('/').collect();
        let path = &parts[3..].join("/");

        Ok(decode_uri(path))
    }

    /// Encodes a `PathBuf` component using `PercentEncode` with UTF-8 charset.
    ///
    /// # Panics
    ///
    /// If the component's `OsStr` representation doesn't belong to valid UTF-8
    /// this function panics.
    fn encode_component(comp: Component) -> String {
        let component = comp
            .as_os_str()
            .to_str()
            .expect("The provided OsStr doesn't belong to the UTF-8 charset.");

        utf8_percent_encode(component, PERCENT_ENCODE_SET).to_string()
    }

    fn breadcrumbs_from_path(root_dir: &Path, path: &Path) -> Result<Vec<BreadcrumbItem>> {
        let root_dir_name = root_dir
            .components()
            .next_back()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("The first path component is not UTF-8 charset compliant.");
        let stripped = path
            .strip_prefix(root_dir)?
            .components()
            .map(Self::encode_component)
            .collect::<Vec<String>>();

        let mut breadcrumbs = stripped
            .iter()
            .enumerate()
            .map(|(idx, entry_name)| BreadcrumbItem {
                depth: (idx + 1) as u8,
                entry_name: percent_decode_str(entry_name)
                    .decode_utf8()
                    .expect("The path name is not UTF-8 compliant")
                    .to_string(),
                entry_link: format!("/{}", stripped[0..=idx].join("/")),
            })
            .collect::<Vec<BreadcrumbItem>>();

        breadcrumbs.insert(
            0,
            BreadcrumbItem {
                depth: 0,
                entry_name: String::from(root_dir_name),
                entry_link: String::from("/"),
            },
        );

        Ok(breadcrumbs)
    }

    /// Creates entry's relative path. Used by Handlebars template engine to
    /// provide navigation through `FileExplorer`
    ///
    /// If the root_dir is: `https-server/src`
    /// The entry path is: `https-server/src/server/service/file_explorer.rs`
    ///
    /// Then the resulting path from this function is the absolute path to
    /// the "entry path" in relation to the "root_dir" path.
    ///
    /// This happens because links should behave relative to the `/` path
    /// which in this case is `http-server/src` instead of system's root path.
    fn make_dir_entry_link(root_dir: &Path, entry_path: &Path) -> String {
        let path = entry_path.strip_prefix(root_dir).unwrap();

        encode_uri(path)
    }

    /// Creates a `DirectoryIndex` with the provided `root_dir` and `path`
    /// (HTTP Request URI)
    fn index_directory(root_dir: PathBuf, path: PathBuf) -> Result<DirectoryIndex> {
        let breadcrumbs = Self::breadcrumbs_from_path(&root_dir, &path)?;
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;

            let display_name = entry
                .file_name()
                .to_str()
                .context("Unable to gather file name into a String")?
                .to_string();

            let date_created = if let Ok(time) = metadata.created() {
                Some(time.into())
            } else {
                None
            };

            let date_modified = if let Ok(time) = metadata.modified() {
                Some(time.into())
            } else {
                None
            };

            let entry_type = if metadata.file_type().is_dir() {
                EntryType::Directory
            } else if let Some(ext) = display_name.split(".").last() {
                match ext.to_ascii_lowercase().as_str() {
                    "gitignore" | "gitkeep" => EntryType::Git,
                    "justfile" => EntryType::Justfile,
                    "md" => EntryType::Markdown,
                    "rs" => EntryType::Rust,
                    "toml" => EntryType::Toml,
                    _ => EntryType::File,
                }
            } else {
                EntryType::File
            };

            directory_entries.push(DirectoryEntry {
                is_dir: metadata.is_dir(),
                size_bytes: metadata.len(),
                entry_path: Self::make_dir_entry_link(&root_dir, &entry.path()),
                display_name,
                entry_type,
                date_created,
                date_modified,
            });
        }

        directory_entries.sort();

        Ok(DirectoryIndex {
            entries: directory_entries,
            breadcrumbs,
            sort: Sort::Directory,
        })
    }

    async fn marshall_directory_index(&self, path: PathBuf) -> Result<DirectoryIndex> {
        Self::index_directory(self.path.clone(), path)
    }
}
