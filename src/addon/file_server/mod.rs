mod directory_entry;
mod file;
mod http_utils;
mod scoped_file_system;

pub use file::{File, FILE_BUFFER_SIZE};
pub use scoped_file_system::{Directory, Entry, ScopedFileSystem};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use http::response::Builder as HttpResponseBuilder;
use http::{StatusCode, Uri};
use hyper::{Body, Response};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use crate::utils::fmt::{format_bytes, format_system_date};
use crate::utils::url_encode::{decode_uri, encode_uri};

use self::directory_entry::{DirectoryEntry, DirectoryIndex};
use self::http_utils::{make_http_file_response, CacheControlDirective};

/// Explorer's Handlebars template filename
const EXPLORER_TEMPLATE: &str = "explorer";

pub struct FileServer {
    root_dir: PathBuf,
    handlebars: Arc<Handlebars<'static>>,
    scoped_file_system: ScopedFileSystem,
}

impl<'a> FileServer {
    /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
    pub fn new(root_dir: PathBuf) -> Self {
        let handlebars = FileServer::make_handlebars_engine();
        let scoped_file_system = ScopedFileSystem::new(root_dir.clone()).unwrap();

        FileServer {
            root_dir,
            handlebars,
            scoped_file_system,
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

        Arc::new(handlebars)
    }

    /// Retrieves the path from the URI and removes the query params
    fn sanitize_path(req_uri: &str) -> Result<PathBuf> {
        let uri = Uri::from_str(req_uri)?;
        let uri_parts = uri.into_parts();

        if let Some(path_and_query) = uri_parts.path_and_query {
            let path = path_and_query.path();

            return decode_uri(path);
        }

        Ok(PathBuf::from_str("/")?)
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
        use std::io::ErrorKind;

        let path = FileServer::sanitize_path(req_path.as_str())?;

        return match self.scoped_file_system.resolve(path).await {
            Ok(entry) => match entry {
                Entry::Directory(dir) => self.render_directory_index(dir.path()).await,
                Entry::File(file) => {
                    make_http_file_response(*file, CacheControlDirective::MaxAge(2500)).await
                }
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(HttpResponseBuilder::new()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(err.to_string()))
                    .expect("Failed to build response")),
                ErrorKind::PermissionDenied => Ok(HttpResponseBuilder::new()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from(err.to_string()))
                    .expect("Failed to build response")),
                _ => Ok(HttpResponseBuilder::new()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(err.to_string()))
                    .expect("Failed to build response")),
            },
        };
    }

    /// Indexes the directory by creating a `DirectoryIndex`. Such `DirectoryIndex`
    /// is used to build the Handlebars "Explorer" template using the Handlebars
    /// engine and builds an HTTP Response containing such file
    async fn render_directory_index(&self, path: PathBuf) -> Result<Response<Body>> {
        let directory_index = FileServer::index_directory(self.root_dir.clone(), path)?;
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

    /// Creates a `DirectoryIndex` with the provided `root_dir` and `path`
    /// (HTTP Request URI)
    fn index_directory(root_dir: PathBuf, path: PathBuf) -> Result<DirectoryIndex> {
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;
            let created_at = if let Ok(time) = metadata.created() {
                format_system_date(time)
            } else {
                String::default()
            };
            let updated_at = if let Ok(time) = metadata.modified() {
                format_system_date(time)
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
                size: format_bytes(metadata.len() as f64),
                entry_path: FileServer::make_dir_entry_link(&root_dir, &entry.path()),
                created_at,
                updated_at,
            });
        }

        directory_entries.sort();

        Ok(DirectoryIndex {
            entries: directory_entries,
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

        encode_uri(&path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::vec;

    use super::FileServer;

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
    fn sanitize_req_uri_path() {
        let have = vec![
            "/index.html",
            "/index.html?foo=1234",
            "/foo/index.html?bar=baz",
            "/foo/bar/baz.html?day=6&month=27&year=2021",
        ];

        let want = vec![
            "/index.html",
            "/index.html",
            "/foo/index.html",
            "/foo/bar/baz.html",
        ];

        for (idx, req_uri) in have.iter().enumerate() {
            let sanitized_path = FileServer::sanitize_path(req_uri).unwrap();
            let wanted_path = PathBuf::from_str(want[idx]).unwrap();

            assert_eq!(sanitized_path, wanted_path);
        }
    }
}
