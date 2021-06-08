use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::utils::terminal::{
    blue_background, green_background, red_background, yellow_background,
};

use super::Print;

pub struct HttpResponseStatus;

impl Print for HttpResponseStatus {
    fn print(&self, _: Arc<Request<Body>>, response: &mut Response<Body>) -> String {
        let status = response.status().as_u16();

        match status {
            0..=199 => blue_background(status.to_string().as_str()),
            200..=299 => green_background(status.to_string().as_str()),
            300..=399 => status.to_string(),
            400..=499 => yellow_background(status.to_string().as_str()),
            500..=599 => red_background(status.to_string().as_str()),
            _ => status.to_string(),
        }
    }
}
