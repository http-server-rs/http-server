pub mod file_explorer;
pub mod file_server;

use anyhow::Result;
use async_trait::async_trait;

use crate::server::{HttpRequest, HttpResponse};

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, req: HttpRequest) -> Result<HttpResponse>;
}
