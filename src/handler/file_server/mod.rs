mod service;
mod utils;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use http::{Method, Response, StatusCode};
use http_body_util::Full;

use crate::handler::Handler;
use crate::server::{HttpRequest, HttpResponse};

pub use crate::handler::file_server::service::FileServerConfig;

use self::service::FileServer as FileServerService;

pub struct FileServer {
    file_service: FileServerService,
}

impl FileServer {
    pub fn new(config: FileServerConfig) -> Self {
        Self {
            file_service: FileServerService::new(config),
        }
    }
}

#[async_trait]
impl Handler for FileServer {
    async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        let (parts, _) = req.into_parts();

        if parts.uri.path().starts_with("/api/v1") {
            let mut response = Response::new(Full::new(Bytes::from("Method Not Allowed")));
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            return Ok(response);
        }

        if parts.method == Method::GET {
            return self.file_service.resolve(parts.uri.to_string()).await;
        }

        let mut response = Response::new(Full::new(Bytes::from("Method Not Allowed")));
        *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        Ok(response)
    }
}
