use http::Method;
use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::utils::terminal::{
    blue_background, green_background, red_background, yellow_background,
};

use super::Print;

pub struct HttpRequestMethod;

impl Print for HttpRequestMethod {
    fn print(&self, request: Arc<Request<Body>>, _: &mut Response<Body>) -> String {
        let method = request.method();

        match *method {
            Method::GET => green_background("GET"),
            Method::POST => blue_background("POST"),
            Method::PUT => yellow_background("PUT"),
            Method::PATCH => yellow_background("PATCH"),
            Method::DELETE => red_background("DELETE"),
            _ => method.to_string(),
        }
    }
}
