use anyhow::{Error, Result};
use clap::ArgMatches;
use std::convert::TryFrom;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use crate::cli::argument;

pub mod file;
pub mod tls;
pub mod util;

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

impl TryFrom<ArgMatches<'static>> for Config {
    type Error = Error;

    fn try_from(matches: ArgMatches<'static>) -> Result<Self, Self::Error> {
        let host = matches.value_of(argument::HOST.name()).unwrap();
        let host = IpAddr::from_str(host)?;

        let port = matches.value_of(argument::PORT.name()).unwrap();
        let port = port.parse::<u16>()?;

        let address = SocketAddr::new(host, port);

        let verbose = matches.is_present(argument::VERBOSE.name());

        let root_dir = if let Some(dir) = matches.value_of(argument::ROOT_DIR.name()) {
            PathBuf::from_str(dir)?
        } else {
            current_dir().unwrap()
        };

        let tls: Option<tls::TlsConfig> = if matches.is_present(argument::TLS.name()) {
            let (cert_path, key_path) = (
                matches.value_of(argument::TLS_CERTIFICATE.name()).unwrap(),
                matches.value_of(argument::TLS_KEY.name()).unwrap(),
            );
            let (cert_path, key_path) =
                (PathBuf::from_str(cert_path)?, PathBuf::from_str(key_path)?);
            let key_algorithm = matches
                .value_of(argument::TLS_KEY_ALGORITHM.name())
                .unwrap();
            let key_algorithm = util::tls::PrivateKeyAlgorithm::from_str(key_algorithm)?;

            Some(tls::TlsConfig::new(cert_path, key_path, key_algorithm)?)
        } else {
            None
        };

        Ok(Config {
            host,
            port,
            address,
            root_dir,
            verbose,
            tls,
        })
    }
}

impl TryFrom<file::ConfigFile> for Config {
    type Error = Error;

    fn try_from(file: file::ConfigFile) -> Result<Self, Self::Error> {
        let host = file.host;
        let port = file.port;
        let address = SocketAddr::new(host, port);
        let verbose = file.verbose;
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
            host,
            port,
            address,
            verbose,
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
