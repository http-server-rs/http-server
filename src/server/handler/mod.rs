mod file_server;

use anyhow::Result;
use futures::Future;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::file_server::FileServer;
use crate::Config;

use super::middleware::Middleware;

use self::file_server::FileServerHandler;

/// The main handler for the HTTP request, a HTTP response is created
/// as a result of this handler.
///
/// This handler will be executed against the HTTP request after every
/// "Middleware Before" chain is executed but before any "Middleware After"
/// chain is executed
pub type Handler = Box<
    dyn Fn(
            Arc<Mutex<Request<Body>>>,
        ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send + Sync>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct HttpHandler {
    file_server_handler: Arc<FileServerHandler>,
    middleware: Arc<Middleware>,
}

impl HttpHandler {
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        let handler = Arc::clone(&self.file_server_handler);
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, handler.handle()).await;

        Ok(response)
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        let file_server = FileServer::new(config.root_dir());
        let file_server_handler = Arc::new(FileServerHandler::new(file_server));
        let middleware = Middleware::try_from(config).unwrap();
        let middleware = Arc::new(middleware);

        HttpHandler {
            file_server_handler,
            middleware,
        }
    }
}
