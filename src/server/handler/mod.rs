mod file_server;
mod proxy;

use anyhow::Result;
use async_trait::async_trait;
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
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        let handler = Arc::clone(&self.request_handler);
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, handler).await;

        Ok(response)
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        if let Some(proxy_config) = config.proxy() {
            let proxy = Proxy::new(&proxy_config.url);
            let request_handler = Arc::new(ProxyHandler::new(proxy));
            let middleware = Middleware::try_from(config).unwrap();
            let middleware = Arc::new(middleware);

            return HttpHandler {
                request_handler,
                middleware,
            };
        }

        let file_server = FileServer::new(config.root_dir(), config.clone());
        let request_handler = Arc::new(FileServerHandler::new(file_server));
        let middleware = Middleware::try_from(config).unwrap();
        let middleware = Arc::new(middleware);

        HttpHandler {
            request_handler,
            middleware,
        }
    }
}
