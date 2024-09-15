use std::pin::Pin;
use std::{future::Future, sync::Arc};

use http_body_util::Full;
use hyper::{body::Bytes, service::Service};

use crate::config::Config;
use crate::middleware::basic_auth::make_basic_auth_middleware;

use super::{HttpRequest, HttpResponse};
use super::middleware::Middleware;

/// Http Request Handler for the Server.
///
/// This struct is responsible for handling incoming HTTP Requests
/// and returning an HTTP Response. Every request is passed through
/// a series of middleware functions before and after handling the
/// request.
pub struct Handler {
    config: Config,
    middleware: Arc<Middleware>,
}

impl From<Config> for Handler {
    fn from(config: Config) -> Self {
        let mut middleware = Middleware::new();

        if let Some(basic_auth) = &config.basic_auth {
            middleware.before(make_basic_auth_middleware(basic_auth));
        }

        Handler { config, middleware: Arc::new(middleware) }
    }
}

impl Service<HttpRequest> for Handler {
    type Response = HttpResponse;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: HttpRequest) -> Self::Future {
        let middleware = Arc::clone(&self.middleware);

        Box::pin(async move {
            match middleware.handle_before(request).await {
                Ok(_request) => {
                    let response = hyper::Response::builder()
                        .body(Full::new(Bytes::from("Hello, World!")))
                        .unwrap();

                    match middleware.handle_after(response).await {
                        Ok(response) => {
                            Ok(response)
                        },
                        Err(response) => {
                            Ok(response)
                        }
                    }
                },
                Err(response) => {
                    Ok(response)
                }
            }
        })
    }
}
