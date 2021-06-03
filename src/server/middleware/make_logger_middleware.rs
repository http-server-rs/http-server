use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::config::logger::LoggerConfig;

use super::MiddlewareAfter;

pub fn make_logger_middleware(logger_config: LoggerConfig) -> MiddlewareAfter {
    let logger = logger_config.logger;

    Box::new(
        move |request: Arc<Request<Body>>, response: &mut Response<Body>| {
            logger.print(request, response);
        },
    )
}
