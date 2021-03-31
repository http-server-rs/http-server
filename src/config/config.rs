use anyhow::{Error, Result};
use clap::ArgMatches;
use std::convert::TryFrom;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use super::ConfigFile;

/// Server instance configuration used on initialization
#[derive(Debug, Clone)]
pub struct Config {
    address: SocketAddr,
    host: IpAddr,
    port: u16,
    root_dir: PathBuf,
    verbose: bool,
}

impl Config {
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn root_dir(&self) -> PathBuf {
        self.root_dir.clone()
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

impl Default for Config {
    fn default() -> Self {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(host, port);
        let root_dir = current_dir().unwrap();

        Self {
            host,
            port,
            address,
            root_dir,
            verbose: false,
        }
    }
}

impl TryFrom<ArgMatches<'static>> for Config {
    type Error = Error;

    fn try_from(matches: ArgMatches<'static>) -> Result<Self, Self::Error> {
        let host = matches.value_of("host").unwrap();
        let host = IpAddr::from_str(host)?;

        let port = matches.value_of("port").unwrap();
        let port = port.parse::<u16>()?;

        let address = SocketAddr::new(host, port);

        let verbose = matches.is_present("verbose");

        let root_dir = if let Some(dir) = matches.value_of("root_dir") {
            PathBuf::from_str(dir)?
        } else {
            current_dir().unwrap()
        };

        Ok(Config {
            host,
            port,
            address,
            root_dir,
            verbose,
        })
    }
}

impl From<ConfigFile> for Config {
    fn from(file: ConfigFile) -> Self {
        let host = file.host;
        let port = file.port;
        let address = SocketAddr::new(host, port);
        let verbose = file.verbose;
        let root_dir = file.root_dir.unwrap_or_default();

        Config {
            host,
            port,
            address,
            verbose,
            root_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_config() {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
        let config = Config::default();

        assert_eq!(
            config.host,
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "default host: {}",
            host
        );
        assert_eq!(config.port, 7878, "default port: {}", port);
        assert_eq!(
            config.address, address,
            "default socket address: {}",
            address
        );
        assert!(!config.verbose, "verbose is off by default");
    }
}
