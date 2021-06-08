use hyper::{Body, Request, Response};
use std::sync::Arc;

use super::Print;

pub struct HttpRequestURI;

impl Print for HttpRequestURI {
    fn print(&self, request: Arc<Request<Body>>, _: &mut Response<Body>) -> String {
        request.uri().to_string()
    }
}
