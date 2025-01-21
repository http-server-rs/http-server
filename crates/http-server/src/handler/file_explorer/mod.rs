use anyhow::Result;
use bytes::Bytes;
use http::Response;
use http_body_util::Full;

use crate::server::{HttpRequest, HttpResponse};

pub struct FileExplorer;

impl FileExplorer {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        let res = Response::new(Full::new(Bytes::from("Hello, World!")));
        Ok(res)
    }
}
