use async_trait::async_trait;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::file_server::FileServer;

use super::RequestHandler;

pub struct FileServerHandler {
    file_server: Arc<FileServer>,
}

impl FileServerHandler {
    pub fn new(file_server: FileServer) -> Self {
        let file_server = Arc::new(file_server);

        FileServerHandler { file_server }
    }
}

#[async_trait]
impl RequestHandler for FileServerHandler {
    async fn handle(&self, req: Arc<Mutex<Request<Body>>>) -> Arc<Mutex<http::Response<Body>>> {
        let request_lock = req.lock().await;
        let req_path = request_lock.uri().to_string();
        let req_method = request_lock.method();

        if req_method == Method::GET {
            let response = self.file_server.resolve(req_path).await.unwrap();

            return Arc::new(Mutex::new(response));
        }

        Arc::new(Mutex::new(
            HttpResponseBuilder::new()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .expect("Unable to build response"),
        ))
    }
}
