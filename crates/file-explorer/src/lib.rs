mod fs;
mod http;

use std::convert::Infallible;
use std::path::PathBuf;
use std::task::Poll;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use futures::future::BoxFuture;
use thiserror::Error;
use tower::Service;

use self::fs::open;
use self::http::{make_http_file_response, CacheControlDirective};

#[derive(Debug, Error)]
pub enum FileExplorerError {
    #[error("Failed to open file")]
    OpenFile(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FileExplorerError>;

pub type FileExplorerResponse = Response<Body>;

#[derive(Clone)]
pub struct FileExplorer {
    base_dir: PathBuf,
}

impl FileExplorer {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Create an absolute path from the base directory and the given path.
    fn absolute_path(&self, path: &str) -> PathBuf {
        let path = path.trim().replace("../", "");
        let mut absolute_path = self.base_dir.clone();

        // If the path starts with a slash, it's already absolute.
        if path == "/" || path.is_empty() {
            return absolute_path;
        }

        absolute_path.push(path);
        absolute_path
    }
}

impl Service<Request<Body>> for FileExplorer {
    type Response = FileExplorerResponse;
    type Error = Infallible;
    type Future = BoxFuture<'static, std::result::Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = req.uri().path();
        let entry_path = self.absolute_path(path);

        Box::pin(async move {
            let entry = open(entry_path).await.unwrap();
            let response = match entry {
                fs::Entry::File(file) => {
                    let response =
                        make_http_file_response(*file, CacheControlDirective::NoCache).await;

                    response.unwrap()
                }
                fs::Entry::Directory(_) => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap(),
            };

            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_absolute_path() {
        let explorer = FileExplorer::new(PathBuf::from("/Users/John/Destkop/www"));
        let cases = vec![
            ("", "/Users/John/Destkop/www"),
            ("/", "/Users/John/Destkop/www"),
            ("/ ", "/Users/John/Destkop/www"),
            ("foo", "/Users/John/Destkop/www/foo"),
            ("foo/bar", "/Users/John/Destkop/www/foo/bar"),
            ("foo/bar/", "/Users/John/Destkop/www/foo/bar/"),
            ("../../foo/", "/Users/John/Destkop/www/foo/"),
        ];

        for (rel_path, full_path) in cases {
            assert_eq!(explorer.absolute_path(rel_path), PathBuf::from(full_path));
        }
    }
}
