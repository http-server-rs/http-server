use anyhow::Result;
use hyper::{Body, Request, Response};

use super::handler::HttpHandler;

pub async fn main_service(handler: HttpHandler, req: Request<Body>) -> Result<Response<Body>> {
    handler.handle_request(req).await
}
