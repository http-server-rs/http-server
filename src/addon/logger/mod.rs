mod log;
mod pattern;

use anyhow::Result;
use hyper::{Body, Request, Response};
use std::str::FromStr;
use std::sync::Arc;

use self::log::Log;
use self::pattern::Pattern;

pub struct Logger {
    print_fn: Arc<dyn Fn(Arc<Request<Body>>, &mut Response<Body>) -> String + Send + Sync>,
    log: Log,
}

impl Logger {
    pub fn new(pattern: &str) -> Result<Self> {
        let pattern = Pattern::from_str(pattern)?;
        let print_fn = Logger::make_print_fn(pattern);
        let print_fn = Arc::new(print_fn);
        let log = Log {};

        Ok(Logger { print_fn, log })
    }

    fn make_print_fn(
        pattern: Pattern,
    ) -> Box<dyn Fn(Arc<Request<Body>>, &mut Response<Body>) -> String + Send + Sync> {
        let print_fn = move |request: Arc<Request<Body>>, response: &mut Response<Body>| {
            pattern.output_string(request, response)
        };

        Box::new(print_fn)
    }

    pub fn print(&self, request: Arc<Request<Body>>, response: &mut Response<Body>) -> Result<()> {
        self.log
            .print((self.print_fn)(request, response).as_bytes())
    }
}
