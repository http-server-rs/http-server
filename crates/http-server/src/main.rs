pub mod cli;
pub mod config;
pub mod plugin;
pub mod server;

use std::{process::exit, sync::Arc};

use anyhow::Result;
use tokio::runtime::Builder;

use self::server::Server;

fn main() -> Result<()> {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .thread_name("http-server")
        .build()?;
    let rt = Arc::new(rt);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    rt.block_on(async {
        match Server::run(Arc::clone(&rt)).await {
            Ok(_) => {
                println!("Server exited successfuly");
                Ok(())
            }
            Err(error) => {
                eprint!("{:?}", error);
                exit(1);
            }
        }
    })
}
