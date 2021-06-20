use http::{Request, Response};
use hyper::Body;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::compression::gzip::compress_http_response;

use super::MiddlewareAfter;

pub fn make_gzip_compression_middleware() -> MiddlewareAfter {
    Box::new(
        move |request: Arc<Request<Body>>, response: Arc<Mutex<Response<Body>>>| {
            Box::pin(async move { compress_http_response(request, response).await })
        },
    )
}
