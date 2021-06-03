mod output_pattern;

use anyhow::Result;
use hyper::{Body, Request, Response};
use std::sync::Arc;
use std::{fmt::Debug, str::FromStr};

use self::output_pattern::OutputPattern;

pub trait Log: Debug {
    fn log(&self) -> String;
}

pub struct Logger {
    output_pattern: OutputPattern,
    print_fn: Arc<dyn Fn(Arc<Request<Body>>, &mut Response<Body>) -> String + Send + Sync>,
}

impl Logger {
    pub fn new(pattern: &str) -> Result<Self> {
        let output_pattern = OutputPattern::from_str(pattern)?;
        let print_fn = Logger::make_print_fn();
        let print_fn = Arc::new(print_fn);

        Ok(Logger {
            output_pattern,
            print_fn,
        })
    }

    fn make_print_fn(
    ) -> Box<dyn Fn(Arc<Request<Body>>, &mut Response<Body>) -> String + Send + Sync> {
        let print_fn = |request: Arc<Request<Body>>, response: &mut Response<Body>| {
            let (uri, method) = (request.uri().to_string(), request.method().to_string());
            let status_code = response.status().to_string();

            format!("{}\t{}\t{}", uri, method, status_code)
        };

        Box::new(print_fn)
    }

    pub fn print(&self, request: Arc<Request<Body>>, response: &mut Response<Body>) {
        let output = (self.print_fn)(request, response);

        println!("-> {}", output);
    }
}
