use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::config::Config;

use super::MiddlewareAfter;

pub fn make_logger_middleware(_: Config) -> MiddlewareAfter {
    Box::new(
        move |request: Arc<Request<Body>>, response: &mut Response<Body>| {
            let (uri, method) = (request.uri().to_string(), request.method().to_string());
            let status_code = response.status().to_string();

            println!("{}\t{}\t{}", uri, method, status_code);
        },
    )
}
