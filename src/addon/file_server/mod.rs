mod directory_entry;
mod file;
mod http_utils;
mod query_params;
mod scoped_file_system;

use chrono::{DateTime, Local};

pub use file::{File, FILE_BUFFER_SIZE};
use humansize::{format_size, DECIMAL};
pub use scoped_file_system::{Directory, Entry, ScopedFileSystem};

use anyhow::{Context, Result};
use handlebars::{handlebars_helper, Handlebars};
use http::response::Builder as HttpResponseBuilder;
use http::{StatusCode, Uri};
use hyper::{Body, Response};
use percent_encoding::{percent_decode_str, utf8_percent_encode};
use std::fs::read_dir;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use crate::config::Config;
use crate::utils::fmt::{format_bytes, format_system_date};
use crate::utils::url_encode::{decode_uri, encode_uri, PERCENT_ENCODE_SET};

use self::directory_entry::{BreadcrumbItem, DirectoryEntry, DirectoryIndex, Sort};
use self::http_utils::{make_http_file_response, CacheControlDirective};
use self::query_params::{QueryParams, SortBy};

/// Explorer's Handlebars template filename
const EXPLORER_TEMPLATE: &str = "explorer";

pub struct FileServer {
    root_dir: PathBuf,
    handlebars: Arc<Handlebars<'static>>,
    scoped_file_system: ScopedFileSystem,
    config: Arc<Config>,
}

impl<'a> FileServer {
    /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
    pub fn new(root_dir: PathBuf, config: Arc<Config>) -> Self {
        let handlebars = FileServer::make_handlebars_engine();
        let scoped_file_system = ScopedFileSystem::new(root_dir.clone()).unwrap();

        FileServer {
            root_dir,
            handlebars,
            scoped_file_system,
            config,
        }
    }

    /// Creates a new `Handlebars` instance with templates registered
    fn make_handlebars_engine() -> Arc<Handlebars<'a>> {
        let mut handlebars = Handlebars::new();

        let template = std::include_bytes!("./template/explorer.hbs");
        let template = std::str::from_utf8(template).unwrap();

        handlebars
            .register_template_string(EXPLORER_TEMPLATE, template)
            .unwrap();

        handlebars_helper!(date: |d: Option<DateTime<Local>>| {
            match d {
                Some(d) => d.format("%Y/%m/%d %H:%M:%S").to_string(),
                None => "-".to_owned(),
            }
        });
        handlebars.register_helper("date", Box::new(date));

        handlebars_helper!(size: |bytes: u64| format_size(bytes, DECIMAL));
        handlebars.register_helper("size", Box::new(size));

        handlebars_helper!(sort_name: |sort: Sort| sort == Sort::Name);
        handlebars.register_helper("sort_name", Box::new(sort_name));

        handlebars_helper!(sort_size: |sort: Sort| sort == Sort::Size);
        handlebars.register_helper("sort_size", Box::new(sort_size));

        handlebars_helper!(sort_date_created: |sort: Sort| sort == Sort::DateCreated);
        handlebars.register_helper("sort_date_created", Box::new(sort_date_created));

        handlebars_helper!(sort_date_modified: |sort: Sort| sort == Sort::DateModified);
        handlebars.register_helper("sort_date_modified", Box::new(sort_date_modified));

        Arc::new(handlebars)
    }

    fn parse_path(req_uri: &str) -> Result<(PathBuf, Option<QueryParams>)> {
        let uri = Uri::from_str(req_uri)?;
        let uri_parts = uri.into_parts();

        if let Some(path_and_query) = uri_parts.path_and_query {
            let path = path_and_query.path();
            let query_params = if let Some(query_str) = path_and_query.query() {
                Some(QueryParams::from_str(query_str)?)
            } else {
                None
            };

            return Ok((decode_uri(path), query_params));
        }

        Ok((PathBuf::from_str("/")?, None))
    }

    /// Resolves a HTTP Request to a file or directory.
    ///
    /// If the method of the HTTP Request is not `GET`, then responds with
    /// `Bad Request`
    ///
    /// If URI doesn't matches a path relative to `root_dir`, then responds
    /// with `Bad Reuest`
    ///
    /// If the HTTP Request URI points to `/` (root), the default behavior
    /// would be to respond with `Not Found` but in order to provide `root_dir`
    /// indexing, the request is handled and renders `root_dir` directory
    /// listing instead.
    ///
    /// If the HTTP Request doesn't match any file relative to `root_dir` then
    /// responds with 'Not Found'
    ///
    /// If the HTTP Request matches a forbidden file (User doesn't have
    /// permissions to read), responds with `Forbidden`
    ///
    /// If the matched path resolves a directory, responds with the directory
    /// listing page
    ///
    /// If the matched path resolves to a file, attempts to render it if the
    /// MIME type is supported, otherwise returns the binary (downloadable file)
    pub async fn resolve(&self, req_path: String) -> Result<Response<Body>> {
        let (path, query_params) = FileServer::parse_path(req_path.as_str())?;

        match self.scoped_file_system.resolve(path).await {
            Ok(entry) => match entry {
                Entry::Directory(dir) => 'dir: {
                    if self.config.use_index() {
                        let mut filepath = dir.path();

                        filepath.push("index.html");
                        if let Ok(file) = tokio::fs::File::open(&filepath).await {
                            break 'dir make_http_file_response(
                                File {
                                    path: filepath,
                                    metadata: file.metadata().await?,
                                    file,
                                },
                                CacheControlDirective::MaxAge(2500),
                            )
                            .await;
                        }
                    }
                    self.render_directory_index(dir.path(), query_params).await
                }
                Entry::File(file) => {
                    make_http_file_response(*file, CacheControlDirective::MaxAge(2500)).await
                }
            },
            Err(err) => {
                if self.config.spa() {
                    make_http_file_response(
                        {
                            let mut path = self.config.root_dir();
                            path.push("index.html");

                            let file = tokio::fs::File::open(&path).await?;

                            let metadata = file.metadata().await?;

                            File {
                                path,
                                metadata,
                                file,
                            }
                        },
                        CacheControlDirective::MaxAge(2500),
                    )
                    .await
                } else {
                    let status = match err.kind() {
                        std::io::ErrorKind::NotFound => hyper::StatusCode::NOT_FOUND,
                        std::io::ErrorKind::PermissionDenied => hyper::StatusCode::FORBIDDEN,
                        _ => hyper::StatusCode::BAD_REQUEST,
                    };

                    let code = match err.kind() {
                        std::io::ErrorKind::NotFound => "404",
                        std::io::ErrorKind::PermissionDenied => "403",
                        _ => "400",
                    };

                    let response = hyper::Response::builder()
                        .status(status)
                        .header(http::header::CONTENT_TYPE, "text/html")
                        .body(hyper::Body::from(
                            handlebars::Handlebars::new().render_template(
                                include_str!("./template/error.hbs"),
                                &serde_json::json!({"error": err.to_string(), "code": code}),
                            )?,
                        ))?;

                    Ok(response)
                }
            }
        }
    }

    /// Indexes the directory by creating a `DirectoryIndex`. Such `DirectoryIndex`
    /// is used to build the Handlebars "Explorer" template using the Handlebars
    /// engine and builds an HTTP Response containing such file
    async fn render_directory_index(
        &self,
        path: PathBuf,
        query_params: Option<QueryParams>,
    ) -> Result<Response<Body>> {
        let directory_index =
            FileServer::index_directory(self.root_dir.clone(), path, query_params)?;
        let html = self
            .handlebars
            .render(EXPLORER_TEMPLATE, &directory_index)
            .unwrap();

        let body = Body::from(html);

        Ok(HttpResponseBuilder::new()
            .header(http::header::CONTENT_TYPE, "text/html")
            .status(StatusCode::OK)
            .body(body)
            .expect("Failed to build response"))
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
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("The first path component is not UTF-8 charset compliant.");
        let stripped = path
            .strip_prefix(root_dir)?
            .components()
            .map(FileServer::encode_component)
            .collect::<Vec<String>>();

        let mut breadcrumbs = stripped
            .iter()
            .enumerate()
            .map(|(idx, entry_name)| BreadcrumbItem {
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
                entry_name: String::from(root_dir_name),
                entry_link: String::from("/"),
            },
        );

        Ok(breadcrumbs)
    }

    /// Creates a `DirectoryIndex` with the provided `root_dir` and `path`
    /// (HTTP Request URI)
    fn index_directory(
        root_dir: PathBuf,
        path: PathBuf,
        query_params: Option<QueryParams>,
    ) -> Result<DirectoryIndex> {
        let breadcrumbs = FileServer::breadcrumbs_from_path(&root_dir, &path)?;
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;
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

            directory_entries.push(DirectoryEntry {
                display_name: entry
                    .file_name()
                    .to_str()
                    .context("Unable to gather file name into a String")?
                    .to_string(),
                is_dir: metadata.is_dir(),
                size_bytes: metadata.len(),
                entry_path: FileServer::make_dir_entry_link(&root_dir, &entry.path()),
                date_created,
                date_modified,
            });
        }

        if let Some(query_params) = query_params {
            if let Some(sort_by) = query_params.sort_by {
                match sort_by {
                    SortBy::Name => {
                        directory_entries.sort_by_key(|entry| entry.display_name.clone());
                    }
                    SortBy::Size => directory_entries.sort_by_key(|entry| entry.size_bytes),
                    SortBy::DateCreated => {
                        directory_entries.sort_by_key(|entry| entry.date_created)
                    }
                    SortBy::DateModified => {
                        directory_entries.sort_by_key(|entry| entry.date_modified)
                    }
                };

                let sort_enum = match sort_by {
                    SortBy::Name => Sort::Name,
                    SortBy::Size => Sort::Size,
                    SortBy::DateCreated => Sort::DateCreated,
                    SortBy::DateModified => Sort::DateModified,
                };

                return Ok(DirectoryIndex {
                    entries: directory_entries,
                    breadcrumbs,
                    sort: sort_enum,
                });
            }
        }

        directory_entries.sort();

        Ok(DirectoryIndex {
            entries: directory_entries,
            breadcrumbs,
            sort: Sort::Directory,
        })
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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::vec;

    use super::{BreadcrumbItem, FileServer};

    #[test]
    fn makes_dir_entry_link() {
        let root_dir = PathBuf::from_str("/Users/bob/sources/http-server").unwrap();
        let entry_path =
            PathBuf::from_str("/Users/bob/sources/http-server/src/server/service/testing stuff/filename with spaces.txt")
                .unwrap();

        assert_eq!(
            "/src/server/service/testing%20stuff/filename%20with%20spaces.txt",
            FileServer::make_dir_entry_link(&root_dir, &entry_path)
        );
    }

    #[test]
    fn parse_req_uri_path() {
        let have = [
            "/index.html",
            "/index.html?foo=1234",
            "/foo/index.html?bar=baz",
            "/foo/bar/baz.html?day=6&month=27&year=2021",
        ];

        let want = [
            "/index.html",
            "/index.html",
            "/foo/index.html",
            "/foo/bar/baz.html",
        ];

        for (idx, req_uri) in have.iter().enumerate() {
            let sanitized_path = FileServer::parse_path(req_uri).unwrap().0;
            let wanted_path = PathBuf::from_str(want[idx]).unwrap();

            assert_eq!(sanitized_path, wanted_path);
        }
    }

    #[test]
    fn breadcrumbs_from_paths() {
        let root_dir = PathBuf::from_str("/Users/bob/sources/http-server").unwrap();
        let entry_path =
            PathBuf::from_str("/Users/bob/sources/http-server/src/server/service/testing stuff/filename with spaces.txt")
                .unwrap();
        let breadcrumbs = FileServer::breadcrumbs_from_path(&root_dir, &entry_path).unwrap();
        let expect = vec![
            BreadcrumbItem {
                entry_name: String::from("http-server"),
                entry_link: String::from("/"),
            },
            BreadcrumbItem {
                entry_name: String::from("src"),
                entry_link: String::from("/src"),
            },
            BreadcrumbItem {
                entry_name: String::from("server"),
                entry_link: String::from("/src/server"),
            },
            BreadcrumbItem {
                entry_name: String::from("service"),
                entry_link: String::from("/src/server/service"),
            },
            BreadcrumbItem {
                entry_name: String::from("testing stuff"),
                entry_link: String::from("/src/server/service/testing%20stuff"),
            },
            BreadcrumbItem {
                entry_name: String::from("filename with spaces.txt"),
                entry_link: String::from(
                    "/src/server/service/testing%20stuff/filename%20with%20spaces.txt",
                ),
            },
        ];

        assert_eq!(breadcrumbs, expect);
    }
}
