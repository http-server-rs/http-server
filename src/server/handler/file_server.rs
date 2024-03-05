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
            let response = match self.file_server.resolve(req_path).await {
                Ok(response) => response,
                Err(err) => hyper::Response::builder()
                    .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                    .header(http::header::CONTENT_TYPE, "text/html")
                    .body(hyper::Body::from(
                        handlebars::Handlebars::new()
                            .render_template(
                                include_str!("../../addon/file_server/template/error.hbs"),
                                &serde_json::json!({"error": err.to_string(), "code": 500}),
                            )
                            .unwrap(),
                    ))
                    .unwrap(),
            };

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
