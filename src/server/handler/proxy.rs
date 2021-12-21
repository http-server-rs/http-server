use async_trait::async_trait;
use hyper::{Body, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::proxy::Proxy;

use super::RequestHandler;

pub struct ProxyHandler {
    proxy: Arc<Proxy>,
}

impl ProxyHandler {
    pub fn new(proxy: Proxy) -> Self {
        let proxy = Arc::new(proxy);

        ProxyHandler { proxy }
    }
}

#[async_trait]
impl RequestHandler for ProxyHandler {
    async fn handle(&self, req: Arc<Mutex<Request<Body>>>) -> Arc<Mutex<http::Response<Body>>> {
        let proxy = Arc::clone(&self.proxy);
        let request = Arc::clone(&req);
        let response = proxy.handle(request).await;

        Arc::new(Mutex::new(response))
    }
}
