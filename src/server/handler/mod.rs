mod file_server;
mod proxy;

use async_trait::async_trait;
use color_eyre::eyre::Context;
use color_eyre::Report;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::file_server::FileServer;
use crate::addon::proxy::Proxy;
use crate::Config;

use super::middleware::Middleware;

use self::file_server::FileServerHandler;
use self::proxy::ProxyHandler;

/// A trait to implement on addons in order to handle incomming requests and
/// generate responses.
#[async_trait]
pub trait RequestHandler {
    async fn handle(&self, req: Arc<Mutex<Request<Body>>>) -> Arc<Mutex<http::Response<Body>>>;
}

#[derive(Clone)]
pub struct HttpHandler {
    request_handler: Arc<dyn RequestHandler + Send + Sync>,
    middleware: Arc<Middleware>,
}

impl HttpHandler {
    pub async fn handle_request(
        self,
        request: Request<Body>,
    ) -> color_eyre::Result<Response<Body>> {
        let handler = Arc::clone(&self.request_handler);
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, handler).await;

        Ok(response)
    }
}

impl TryFrom<Arc<Config>> for HttpHandler {
    type Error = Report;

    fn try_from(config: Arc<Config>) -> Result<Self, Self::Error> {
        if let Some(proxy_config) = config.proxy.clone() {
            let proxy = Proxy::new(proxy_config.uri);
            let request_handler = Arc::new(ProxyHandler::new(proxy));
            let middleware = Middleware::from(config);
            let middleware = Arc::new(middleware);

            return Ok(HttpHandler {
                request_handler,
                middleware,
            });
        }

        let file_server =
            FileServer::new(config.clone()).context("Failed to create file server")?;
        let request_handler = Arc::new(FileServerHandler::new(file_server));
        let middleware = Middleware::from(config);
        let middleware = Arc::new(middleware);

        Ok(HttpHandler {
            request_handler,
            middleware,
        })
    }
}
