use anyhow::Result;
use clap::ArgMatches;
use std::{convert::TryFrom, path::PathBuf, str::FromStr};

use crate::config::{Config, ConfigFile};

pub mod cli;
pub mod config;

fn resolve_config(matches: ArgMatches<'static>) -> Result<Config> {
    if matches.is_present("config") {
        let file_path = matches.value_of("config").unwrap();
        let file_path = PathBuf::from_str(file_path)?;
        let config_file = ConfigFile::from_file(Some(file_path))?;
        let config = Config::from(config_file);

        return Ok(config);
    }

    Config::try_from(matches)
}

pub fn run() -> Result<()> {
    let cli = cli::build();
    let matches = cli.get_matches();
    let config = resolve_config(matches)?;

    println!("{:?}", config);

    Ok(())
}
