use anyhow::Result;
use clap::ArgMatches;
use std::{convert::TryFrom, path::PathBuf, str::FromStr};

use crate::config::file::ConfigFile;
use crate::config::Config;

mod cli;
mod config;
mod server;

fn resolve_config(matches: ArgMatches<'static>) -> Result<Config> {
    if matches.is_present("config") {
        // If theres a `config` file path present we want to read the config
        // from that file
        let file_path = matches.value_of("config").unwrap();
        let file_path = PathBuf::from_str(file_path)?;
        let config_file = ConfigFile::from_file(Some(file_path))?;
        let config = Config::from(config_file);

        return Ok(config);
    }

    // Otherwise configuration is build from CLI arguments
    Config::try_from(matches)
}

pub async fn run() -> Result<()> {
    let cli = cli::build();
    let matches = cli.get_matches();
    let config = resolve_config(matches)?;
    let server = server::Server::from(config);

    server.serve().await;

    Ok(())
}
