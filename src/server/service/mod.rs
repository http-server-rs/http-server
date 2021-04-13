mod file_explorer;

pub use file_explorer::*;

use anyhow::Result;
use hyper::{Body, Request, Response};

use super::handler::Handler;

pub async fn main_service(handler: Handler, req: Request<Body>) -> Result<Response<Body>> {
    handler.handle_request(req).await
}
