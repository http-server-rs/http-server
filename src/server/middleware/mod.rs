pub mod cors;
pub mod gzip;

use anyhow::{Error, Result};
use futures::Future;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;

use self::cors::make_cors_middleware;
use self::gzip::make_gzip_compression_middleware;

pub type MiddlewareBefore = Box<dyn Fn(&mut Request<Body>) + Send + Sync>;
pub type MiddlewareAfter = Box<
    dyn Fn(
            Arc<Request<Body>>,
            Arc<Mutex<Response<Body>>>,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync>>
        + Send
        + Sync,
>;
pub type Handler = Box<
    dyn Fn(Arc<Request<Body>>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + Sync>>
        + Send
        + Sync,
>;

pub struct Middleware {
    before: Vec<MiddlewareBefore>,
    after: Vec<MiddlewareAfter>,
}

impl Middleware {
    /// Appends a middleware function to run before handling the
    /// HTTP Request
    #[allow(dead_code)]
    pub fn before(&mut self, middleware: MiddlewareBefore) {
        self.before.push(middleware);
    }

    /// Appends a middleware function to run after handling the
    /// HTTP Request. Thus, functions appended after will receive
    /// the handler's HTTP Response instead
    pub fn after(&mut self, middleware: MiddlewareAfter) {
        self.after.push(middleware);
    }

    /// Runs functions from the chain that must run before
    /// executing the handler (applied to the HTTP Request).
    /// Then performs the handler operations on the HTTP Request
    /// and finally executes the functions on the "after" chain
    /// with the HTTP Response from the handler
    pub async fn handle(&self, mut request: Request<Body>, handler: Handler) -> Response<Body> {
        for fx in self.before.iter() {
            fx(&mut request);
        }

        let request = Arc::new(request);
        let response = handler(Arc::clone(&request)).await;
        let response = Arc::new(Mutex::new(response));

        for fx in self.after.iter() {
            if let Err(error) = fx(Arc::clone(&request), Arc::clone(&response)).await {
                eprintln!("{:?}", error);
            }
        }

        Arc::try_unwrap(response)
            .expect("There's one or more reference/s being hold by a middleware chain.")
            .into_inner()
    }
}

impl Default for Middleware {
    fn default() -> Self {
        Middleware {
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}

impl TryFrom<Arc<Config>> for Middleware {
    type Error = Error;

    fn try_from(config: Arc<Config>) -> Result<Self, Self::Error> {
        let mut middleware = Middleware::default();

        if let Some(cors_config) = config.cors() {
            let cors_middleware = make_cors_middleware(cors_config);

            middleware.after(cors_middleware);
        }

        if let Some(compression_config) = config.compression() {
            if compression_config.gzip {
                middleware.after(make_gzip_compression_middleware());
            }
        }

        Ok(middleware)
    }
}
