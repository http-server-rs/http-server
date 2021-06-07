use hyper::{Body, Request, Response};
use std::sync::Arc;

use super::Print;

pub struct DateTime;

impl Print for DateTime {
    fn print(&self, _: Arc<Request<Body>>, _: &mut Response<Body>) -> String {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
