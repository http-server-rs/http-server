use hyper::{Body, Request, Response};
use std::sync::Arc;

use super::Print;

pub struct HttpRequestIP;

impl Print for HttpRequestIP {
    fn print(&self, request: Arc<Request<Body>>, _: &mut Response<Body>) -> String {
        String::from("127.0.0.1")
    }
}
