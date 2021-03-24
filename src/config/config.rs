use anyhow::{Error, Result};
use clap::ArgMatches;
use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use super::ConfigFile;

/// Server instance configuration used on initialization
#[derive(Debug)]
pub struct Config {
    host: IpAddr,
    port: u16,
    address: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(host, port);

        Self {
            host,
            port,
            address,
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

        Ok(Config {
            host,
            port,
            address,
        })
    }
}

impl From<ConfigFile> for Config {
    fn from(file: ConfigFile) -> Self {
        let host = file.host;
        let port = file.port;
        let address = SocketAddr::new(host, port);

        Config {
            host,
            port,
            address,
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
        )
    }
}
