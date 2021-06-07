use hyper::{Body, Request, Response};
use std::sync::Arc;

use super::Print;

pub struct HttpResponseStatus;

impl Print for HttpResponseStatus {
    fn print(&self, _: Arc<Request<Body>>, response: &mut Response<Body>) -> String {
        response.status().as_u16().to_string()
    }
}
