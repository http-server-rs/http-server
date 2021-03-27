use anyhow::Result;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Request, Response};
use hyper_staticfile::{resolve, ResolveResult, ResponseBuilder as FileResponseBuilder};
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileExplorer {
    root_dir: PathBuf,
    cache_headers: Option<u32>,
}

impl FileExplorer {
    pub fn new(root_dir: PathBuf) -> Self {
        FileExplorer {
            root_dir,
            cache_headers: None,
        }
    }

    pub async fn resolve(&self, req: Request<Body>) -> Result<Response<Body>> {
        match resolve(&self.root_dir, &req).await.unwrap() {
            ResolveResult::MethodNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::UriNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::NotFound => Ok(HttpResponseBuilder::new()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::PermissionDenied => Ok(HttpResponseBuilder::new()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::IsDirectory => Ok(HttpResponseBuilder::new()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::Found(file, metadata, mime) => Ok(FileResponseBuilder::new()
                .request(&req)
                .cache_headers(self.cache_headers)
                .build(ResolveResult::Found(file, metadata, mime))
                .expect("Failed to build response")),
        }
    }
}
