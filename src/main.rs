pub mod cli;
pub mod server;

use std::process::exit;

use anyhow::Result;

use crate::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    match Server::run().await {
        Ok(_) => {
            println!("Server exited successfuly");
            Ok(())
        }
        Err(error) => {
            eprint!("{:?}", error);
            exit(1);
        }
    }
}
