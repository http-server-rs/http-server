use std::process::exit;

use anyhow::{Context, Result};
use http_server_lib::server::Server;
use structopt::StructOpt;

#[cfg(feature = "dhat-profiling")]
use dhat::{Dhat, DhatAlloc};

use http_server_lib::cli::Cli;
use http_server_lib::config::Config;
use http_server_lib::config::file::ConfigFile;

#[cfg(feature = "dhat-profiling")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "dhat-profiling")]
    let _dhat = Dhat::start_heap_profiling();
    let args = Cli::from_args();
    let config = resolve_config(args)?;

    match Server::run(config).await {
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

fn resolve_config(args: Cli) -> Result<Config> {
    if let Some(config_path) = args.config {
        let config_file = ConfigFile::from_file(config_path)?;
        let config = Config::try_from(config_file)?;

        return Ok(config);
    }

    // Otherwise configuration is build from CLI arguments
    Config::try_from(args)
        .with_context(|| anyhow::Error::msg("Failed to parse arguments from stdin"))
}
