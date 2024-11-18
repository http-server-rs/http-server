use std::net::IpAddr;
use std::process::exit;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio::runtime::Builder;
use tracing::{error, info};

use crate::config::Config;
use crate::server::Server;

const THREAD_NAME: &str = "http-server";

#[derive(Debug, Parser)]
pub struct StartOpt {
    /// Host (IP) to bind the server
    #[clap(long, default_value = "0.0.0.0")]
    pub host: IpAddr,
    /// Port to bind the server
    #[clap(short = 'p', long, default_value = "7878")]
    pub port: u16,
}

impl StartOpt {
    pub fn exec(&self) -> Result<()> {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .thread_name(THREAD_NAME)
            .build()?;
        let rt = Arc::new(rt);
        let config = Config {
            host: self.host,
            port: self.port,
        };
        let server = Server::new(config);

        rt.block_on(async {
            match server.run(Arc::clone(&rt)).await {
                Ok(_) => {
                    info!("Server exited successfuly");
                    Ok(())
                }
                Err(error) => {
                    error!(%error, "Server exited with error");
                    exit(1);
                }
            }
        })
    }
}
