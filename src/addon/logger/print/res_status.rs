use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::utils::terminal::{green_background, yellow_background};

use super::Print;

pub struct HttpResponseStatus;

impl Print for HttpResponseStatus {
    fn print(&self, _: Arc<Request<Body>>, response: &mut Response<Body>) -> String {
        let status = response.status().as_u16();

        match status {
            0 | 199 => format!("\x1B[32m{}\x1B[0m", status.to_string().as_str()),
            200 | 299 => green_background(status.to_string().as_str()),
            300 | 399 => format!("\x1B[32m{}\x1B[0m", status.to_string().as_str()),
            400 | 499 => yellow_background(status.to_string().as_str()),
            500 | 599 => format!("\x1B[32m{}\x1B[0m", status.to_string().as_str()),
            _ => status.to_string(),
        }
    }
}
