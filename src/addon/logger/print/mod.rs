pub mod datetime;
pub mod res_delay;
pub mod res_status;

use hyper::{Body, Request, Response};
use std::sync::Arc;

pub trait Print {
    fn print(&self, request: Arc<Request<Body>>, response: &mut Response<Body>) -> String;
}
