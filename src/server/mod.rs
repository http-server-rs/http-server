mod services;

use std::sync::Arc;

use anyhow::Result;
use axum::Router;

use crate::config::Config;
use crate::server::services::file_server::FileServer;

use self::services::proxy::Proxy;

pub struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn new(config: Config) -> Server {
        let config = Arc::new(config);

        Server { config }
    }

    pub async fn run(&self) -> Result<()> {
        let router = self.router();

        axum::Server::bind(&self.config.address())
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        let mut router = Router::new();

        if let Some(proxy_config) = self.config.proxy() {
            let proxy = Proxy::new(&proxy_config.url);

            router = router.nest_service("/", proxy);
            return router;
        }

        let file_server = FileServer::new(self.config.root_dir());

        router = router.nest_service("/", file_server);
        router
    }
}
