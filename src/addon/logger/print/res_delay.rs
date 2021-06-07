use hyper::{Body, Request, Response};
use std::sync::Arc;

use super::Print;

pub struct HttpResponseDelay;

impl Print for HttpResponseDelay {
    fn print(&self, _: Arc<Request<Body>>, _: &mut Response<Body>) -> String {
        String::from("1.02387503s")
    }
}
