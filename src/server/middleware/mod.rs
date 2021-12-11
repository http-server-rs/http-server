pub mod basic_auth;
pub mod cors;
pub mod gzip;
pub mod logger;

use anyhow::Error;
use futures::Future;
use hyper::Body;
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::handler::Handler;
use crate::config::Config;

use self::basic_auth::make_basic_auth_middleware;
use self::cors::make_cors_middleware;
use self::gzip::make_gzip_compression_middleware;
use self::logger::make_logger_middleware;

/// Middleware HTTP Response which expands to a `Arc<Mutex<http::Request<T>>>`
pub type Request<T> = Arc<Mutex<http::Request<T>>>;

/// Middleware HTTP Response which expands to a `Arc<Mutex<http::Response<T>>>`
pub type Response<T> = Arc<Mutex<http::Response<T>>>;

/// Middleware chain `Result` which specifies the `Err` variant as a
/// HTTP response.
pub type Result = std::result::Result<(), http::Response<Body>>;

/// Middleware chain to execute before the main handler digests the
/// HTTP request. No HTTP response is available at this point.
pub type MiddlewareBefore =
    Box<dyn Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Result> + Send + Sync>> + Send + Sync>;

/// Middleware chain to execute after the main handler digests the
/// HTTP request. The HTTP response is created by the handler and
/// consumed by every middleware after chain.
pub type MiddlewareAfter = Box<
    dyn Fn(Request<Body>, Response<Body>) -> Pin<Box<dyn Future<Output = Result> + Send + Sync>>
        + Send
        + Sync,
>;

#[derive(Default)]
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
    pub async fn handle(
        &self,
        request: http::Request<Body>,
        handler: Handler,
    ) -> http::Response<Body> {
        let request = Arc::new(Mutex::new(request));

        for fx in self.before.iter() {
            if let Err(err) = fx(Arc::clone(&request)).await {
                return err;
            }
        }

        let response = handler(Arc::clone(&request)).await;
        let response = Arc::new(Mutex::new(response));

        for fx in self.after.iter() {
            if let Err(err) = fx(Arc::clone(&request), Arc::clone(&response)).await {
                return err;
            }
        }

        Arc::try_unwrap(response)
            .expect("There's one or more reference/s being hold by a middleware chain.")
            .into_inner()
    }
}

impl TryFrom<Arc<Config>> for Middleware {
    type Error = Error;

    fn try_from(config: Arc<Config>) -> std::result::Result<Self, Self::Error> {
        let mut middleware = Middleware::default();

        if let Some(basic_auth_config) = config.basic_auth() {
            let basic_auth_middleware = make_basic_auth_middleware(basic_auth_config);

            middleware.before(basic_auth_middleware);
        }

        if let Some(cors_config) = config.cors() {
            let cors_middleware = make_cors_middleware(cors_config);

            middleware.after(cors_middleware);
        }

        if let Some(compression_config) = config.compression() {
            if compression_config.gzip {
                middleware.after(make_gzip_compression_middleware());
            }
        }

        middleware.after(make_logger_middleware());

        Ok(middleware)
    }
}
