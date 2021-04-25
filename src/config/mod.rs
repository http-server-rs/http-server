pub mod cors;
pub mod file;
pub mod tls;
pub mod util;

use anyhow::{Error, Result};
use std::convert::TryFrom;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use crate::cli::Cli;

/// Server instance configuration used on initialization
#[derive(Debug, Clone)]
pub struct Config {
    address: SocketAddr,
    host: IpAddr,
    port: u16,
    root_dir: PathBuf,
    verbose: bool,
    tls: Option<tls::TlsConfig>,
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

    pub fn tls(&self) -> Option<tls::TlsConfig> {
        self.tls.clone()
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
            tls: None,
        }
    }
}

impl TryFrom<Cli> for Config {
    type Error = anyhow::Error;

    fn try_from(cli_aguments: Cli) -> Result<Self, Self::Error> {
        let verbose = cli_aguments.verbose;
        let root_dir = if cli_aguments.root_dir.to_str().unwrap() == "./" {
            current_dir().unwrap()
        } else {
            cli_aguments.root_dir.canonicalize().unwrap()
        };

        let tls: Option<tls::TlsConfig> = if cli_aguments.tls {
            Some(tls::TlsConfig::new(
                cli_aguments.tls_cert,
                cli_aguments.tls_key,
                cli_aguments.tls_key_algorithm,
            )?)
        } else {
            None
        };

        Ok(Config {
            host: cli_aguments.host,
            port: cli_aguments.port,
            address: SocketAddr::new(cli_aguments.host, cli_aguments.port),
            root_dir,
            verbose,
            tls,
        })
    }
}

impl TryFrom<file::ConfigFile> for Config {
    type Error = Error;

    fn try_from(file: file::ConfigFile) -> Result<Self, Self::Error> {
        let root_dir = file.root_dir.unwrap_or_default();
        let tls: Option<tls::TlsConfig> = if let Some(https_config) = file.tls {
            Some(tls::TlsConfig::new(
                https_config.cert,
                https_config.key,
                https_config.key_algorithm,
            )?)
        } else {
            None
        };

        Ok(Config {
            host: file.host,
            port: file.port,
            address: SocketAddr::new(file.host, file.port),
            verbose: file.verbose,
            root_dir,
            tls,
        })
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
