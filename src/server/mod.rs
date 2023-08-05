mod services;

use std::sync::Arc;

use anyhow::Result;
use axum::Router;

use crate::config::Config;
use crate::server::services::file_server::FileServer;

pub struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn new(config: Config) -> Server {
        let config = Arc::new(config);

        Server { config }
    }

    pub async fn run(&self) -> Result<()> {
        let file_server = FileServer::new(self.config.root_dir());
        let router = Router::new().nest_service("/", file_server);

        axum::Server::bind(&self.config.address())
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}
