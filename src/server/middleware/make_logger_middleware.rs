use hyper::{Body, Request, Response};

use crate::config::Config;

use super::{MiddlewareAfter, MiddlewareBefore};

pub fn make_logger_middleware(_: Config) -> (MiddlewareBefore, MiddlewareAfter) {
    let before = Box::new(move |request: &mut Request<Body>| {
        println!("{}", request.method());
    });

    let after = Box::new(move |response: &mut Response<Body>| {
        println!("{}", response.status());
    });

    (before, after)
}
