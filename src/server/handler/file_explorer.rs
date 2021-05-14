use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::{DateTime, Local};
use handlebars::Handlebars;
use http::response::Builder as HttpResponseBuilder;
use http::{Method, StatusCode};
use hyper::{Body, Request, Response};
use hyper_staticfile::{resolve_path, ResolveResult, ResponseBuilder as FileResponseBuilder};
use serde::Serialize;
use std::cmp::{Ord, Ordering};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use crate::server::middleware::Handler;

/// Creates a `middleware::Handler` which makes use of the provided `FileExplorer`
pub fn make_file_explorer_handler(file_explorer: Arc<FileExplorer>) -> Handler {
    Box::new(move |request: Arc<Request<Body>>| {
        let file_explorer = Arc::clone(&file_explorer);
        let req_path = request.uri().to_string();

        Box::pin(async move {
            if request.method() == Method::GET {
                return file_explorer
                    .resolve(req_path)
                    .await
                    .map_err(|e| {
                        HttpResponseBuilder::new()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(e.to_string()))
                            .expect("Unable to build response")
                    })
                    .unwrap();
            }

            HttpResponseBuilder::new()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .expect("Unable to build response")
        })
    })
}

/// Explorer's Handlebars template filename
const EXPLORER_TEMPLATE: &str = "explorer";

/// Byte size units
const BYTE_SIZE_UNIT: [&str; 9] = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

/// A Directory entry used to display a File Explorer's entry.
/// This struct is directly related to the Handlebars template used
/// to power the File Explorer's UI
#[derive(Debug, Eq, Serialize)]
struct DirectoryEntry {
    display_name: String,
    is_dir: bool,
    size: String,
    entry_path: String,
    created_at: String,
    updated_at: String,
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_dir && other.is_dir {
            return self.display_name.cmp(&other.display_name);
        }

        if self.is_dir && !other.is_dir {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_dir && other.is_dir {
            return Some(self.display_name.cmp(&other.display_name));
        }

        if self.is_dir && !other.is_dir {
            return Some(Ordering::Less);
        }

        Some(Ordering::Greater)
    }
}

impl PartialEq for DirectoryEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.is_dir && other.is_dir {
            return self.display_name == other.display_name;
        }

        self.display_name == other.display_name
    }
}

/// The value passed to the Handlebars template engine.
/// All references contained in File Explorer's UI are provided
/// via the `DirectoryIndex` struct
#[derive(Debug, Serialize)]
struct DirectoryIndex {
    /// Directory listing entry
    entries: Vec<DirectoryEntry>,
}

/// The File Explorer service is in charge of indexing and rendering a
/// rich view of a Directory served by the HTTP Server.
///
/// Files and directories rendered should be relative to the configured
/// `root_dir`. This means that if the provided `root_dir` is:
/// `Users/Esteban/Documents` then the HTTP Request URI `/` will match
/// `Users/Esteban/Documents` and HTTP Request URI `/Lists/FavoriteFood.txt`
/// will match `Users/Esteban/Documents/Lists/FavoriteFood.txt`.
///
/// The File Explorer will read the HTTP Request URI and serve either
/// a directory listing UI, for directory/folder matches or a static file
/// for supported MIME types.
///
/// If the matched file is not a supported MIME type, such file will be
/// downloaded based on navigator preferences.
#[derive(Clone)]
pub struct FileExplorer {
    root_dir: PathBuf,
    cache_headers: Option<u32>,
    handlebars: Arc<Handlebars<'static>>,
}

impl<'a> FileExplorer {
    /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
    pub fn new(root_dir: PathBuf) -> Self {
        let handlebars = FileExplorer::make_handlebars_engine();

        FileExplorer {
            root_dir,
            cache_headers: None,
            handlebars,
        }
    }

    /// Creates a new `Handlebars` instance with templates registered
    fn make_handlebars_engine() -> Arc<Handlebars<'a>> {
        let mut handlebars = Handlebars::new();

        let template = std::include_bytes!("../template/explorer.hbs");
        let template = std::str::from_utf8(template).unwrap();

        handlebars
            .register_template_string(EXPLORER_TEMPLATE, template)
            .unwrap();

        Arc::new(handlebars)
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
    /// indexing, the request is handled and renders `root_dir` directory listing
    /// instead.
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
        match resolve_path(&self.root_dir, req_path.as_str())
            .await
            .unwrap()
        {
            ResolveResult::MethodNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::UriNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::NotFound => {
                if req_path == "/" {
                    let directory_path = self
                        .make_absolute_path_from_req_path(req_path.as_str())
                        .unwrap();

                    return self.render_directory_index(directory_path).await;
                }

                Ok(HttpResponseBuilder::new()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Failed to build response"))
            }
            ResolveResult::PermissionDenied => Ok(HttpResponseBuilder::new()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::IsDirectory => {
                let directory_path = self
                    .make_absolute_path_from_req_path(req_path.as_str())
                    .unwrap();

                return self.render_directory_index(directory_path).await;
            }
            ResolveResult::Found(file, metadata, mime) => Ok(FileResponseBuilder::new()
                .cache_headers(self.cache_headers)
                .build(ResolveResult::Found(file, metadata, mime))
                .expect("Failed to build response")),
        }
    }

    /// Indexes the directory by creating a `DirectoryIndex`. Such `DirectoryIndex`
    /// is used to build the Handlebars "Explorer" template using the Handlebars
    /// engine and builds an HTTP Response containing such file
    async fn render_directory_index(&self, path: PathBuf) -> Result<Response<Body>> {
        let directory_index = FileExplorer::index_directory(self.root_dir.clone(), path)?;
        let html = self
            .handlebars
            .render(EXPLORER_TEMPLATE, &directory_index)
            .unwrap();

        let body = Body::from(html);

        Ok(HttpResponseBuilder::new()
            .status(StatusCode::OK)
            .body(body)
            .expect("Failed to build response"))
    }

    /// Creates a `DirectoryIndex` with the provided `root_dir` and `path`
    /// (HTTP Request URI)
    fn index_directory(root_dir: PathBuf, path: PathBuf) -> Result<DirectoryIndex> {
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries.into_iter() {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;
            let created_at = if let Ok(time) = metadata.created() {
                FileExplorer::format_system_date(time)
            } else {
                String::default()
            };
            let updated_at = if let Ok(time) = metadata.modified() {
                FileExplorer::format_system_date(time)
            } else {
                String::default()
            };

            directory_entries.push(DirectoryEntry {
                display_name: entry
                    .file_name()
                    .to_str()
                    .context("Unable to gather file name into a String")?
                    .to_string(),
                is_dir: metadata.is_dir(),
                size: FileExplorer::format_bytes(metadata.len() as f64),
                entry_path: FileExplorer::make_dir_entry_link(&root_dir, &entry.path()),
                created_at,
                updated_at,
            });
        }

        directory_entries.sort();

        Ok(DirectoryIndex {
            entries: directory_entries,
        })
    }

    /// Creates an absolute path by appending the HTTP Request URI to the
    /// `root_dir`
    fn make_absolute_path_from_req_path(&self, req_path: &str) -> Result<PathBuf> {
        let mut root_dir = self.root_dir.clone();
        let req_path = if req_path.starts_with('/') {
            let path = req_path[1..req_path.len()].to_string();

            if path.ends_with('/') {
                return PathBuf::from_str(path[..path.len() - 1].to_string().as_str())
                    .context("Unable to buid path");
            }

            PathBuf::from_str(path.as_str())?
        } else {
            PathBuf::from_str(req_path)?
        };

        root_dir.push(req_path);

        Ok(root_dir)
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
        // format!("/{}", &entry_path[current_dir_path.len()..])
        let root_dir = root_dir.to_str().unwrap();
        let entry_path = entry_path.to_str().unwrap();

        entry_path[root_dir.len()..].to_string()
    }

    /// Calculates the format of the `Bytes` by converting `bytes` to the
    /// corresponding unit and returns a human readable size label
    fn format_bytes(bytes: f64) -> String {
        if bytes == 0. {
            return String::from("0 Bytes");
        }

        let i = (bytes.log10() / 1024_f64.log10()).floor();
        let value = bytes / 1024_f64.powf(i);

        format!("{:.2} {}", value, BYTE_SIZE_UNIT[i as usize])
    }

    /// Formats a `SystemTime` into a YYYY/MM/DD HH:MM:SS time `String`
    fn format_system_date(system_time: SystemTime) -> String {
        let datetime: DateTime<Local> = DateTime::from(system_time);

        format!(
            "{}/{:0>2}/{:0>2} {:0>2}:{:0>2}:{:0>2}",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn formats_bytes() {
        let byte_sizes = vec![1024., 1048576., 1073741824., 1099511627776.];

        let expect = vec![
            String::from("1.00 KB"),
            String::from("1.00 MB"),
            String::from("1.00 GB"),
            String::from("1.00 TB"),
        ];

        for (idx, size) in byte_sizes.into_iter().enumerate() {
            assert_eq!(FileExplorer::format_bytes(size), expect[idx]);
        }
    }

    #[test]
    fn makes_dir_entry_link() {
        let root_dir = PathBuf::from_str("/Users/bob/sources/http-server").unwrap();
        let entry_path =
            PathBuf::from_str("/Users/bob/sources/http-server/src/server/service/file_explorer.rs")
                .unwrap();

        assert_eq!(
            "/src/server/service/file_explorer.rs",
            FileExplorer::make_dir_entry_link(&root_dir, &entry_path)
        );
    }
}
