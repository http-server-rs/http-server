use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use tower::Service;

#[derive(Debug)]
pub struct FileExplorerResponseFuture {
    status_code: StatusCode,
    uri: String,
}

impl Future for FileExplorerResponseFuture {
    type Output = Result<Response<Body>, Infallible>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut res = Response::default();

        *res.status_mut() = self.status_code;
        *res.body_mut() = Body::from(self.uri.clone());

        Poll::Ready(Ok(res))
    }
}

#[derive(Clone)]
pub struct FileExplorer {
    root_dir: PathBuf,
}

impl FileExplorer {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }
}

impl Service<Request<Body>> for FileExplorer {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = FileExplorerResponseFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        tracing::info!("Received request! Attempt to find on {:?}", self.root_dir);

        FileExplorerResponseFuture {
            status_code: StatusCode::OK,
            uri: req.uri().to_string(),
        }
    }
}
