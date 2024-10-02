use std::future::Future;
use std::pin::Pin;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::Service;

use super::{HttpRequest, HttpResponse};

/// Http Request Handler for the Server.
///
/// This struct is responsible for handling incoming HTTP Requests
/// and returning an HTTP Response. Every request is passed through
/// a series of middleware functions before and after handling the
/// request.
pub struct Handler {}

impl Service<HttpRequest> for Handler {
    type Response = HttpResponse;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, _: HttpRequest) -> Self::Future {
        Box::pin(async move {
            Ok(hyper::Response::builder()
                .body(Full::new(Bytes::from("Hello, World!")))
                .unwrap())
        })
    }
}
