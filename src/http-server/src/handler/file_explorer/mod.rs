mod core;
mod proto;
mod utils;

use core::Entry;
use std::fs::read_dir;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures::StreamExt;
use http::HeaderName;
use http::{HeaderValue, Method, Response, StatusCode, Uri, header::CONTENT_TYPE, request::Parts};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use percent_encoding::{percent_decode_str, utf8_percent_encode};
use proto::{DirectoryEntry, DirectoryIndex, EntryType, Sort};
use rust_embed::Embed;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::handler::Handler;
use crate::server::{HttpRequest, HttpResponse};

use self::proto::BreadcrumbItem;
use self::utils::{PERCENT_ENCODE_SET, decode_uri, encode_uri};

const X_FILE_NAME: &str = "x-file-name";
const X_FILE_NAME_HTTP_HEADER: HeaderName = HeaderName::from_static(X_FILE_NAME);

#[derive(Embed)]
#[folder = "./ui"]
struct FileExplorerAssets;

pub struct FileExplorer {
    file_explorer: core::FileExplorer,
    path: PathBuf,
}

impl FileExplorer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_explorer: core::FileExplorer::new(path.clone()),
            path,
        }
    }

    async fn handle_api(&self, parts: Parts, body: Incoming) -> Result<HttpResponse> {
        let path = Self::parse_req_uri(parts.uri.clone())?;

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
            Method::POST => self.handle_file_upload(parts, body).await,
            _ => Ok(Response::new(Full::new(Bytes::from("Unsupported method")))),
        }
    }

    async fn handle_file_upload(&self, parts: Parts, body: Incoming) -> Result<HttpResponse> {
        if let Err(err) = self.process_multipart(body, parts).await {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("INTERNAL SERVER ERROR: {err}")))
                .unwrap());
        }

        Ok(Response::new(Full::from("Success")))
    }

    async fn process_multipart(&self, bytes: Incoming, parts: Parts) -> Result<()> {
        let file_name = parts
            .headers
            .get(X_FILE_NAME_HTTP_HEADER)
            .and_then(|hv| hv.to_str().ok())
            .context(format!("Missing '{X_FILE_NAME}' header"))?;
        let mut stream = bytes.into_data_stream();
        let mut file = File::create(file_name)
            .await
            .context("Failed to create target file for upload.")?;

        while let Some(Ok(bytes)) = stream.next().await {
            file.write_all(&bytes)
                .await
                .context("Failed to write bytes to file")?;
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

#[async_trait]
impl Handler for FileExplorer {
    async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        let (parts, body) = req.into_parts();

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
}
