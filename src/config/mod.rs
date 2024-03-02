pub mod basic_auth;
pub mod compression;
pub mod cors;
pub mod file;
pub mod proxy;
pub mod tls;
pub mod util;

use color_eyre::Report;
use http::Uri;
use std::convert::TryFrom;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use crate::cli::Cli;

use self::basic_auth::BasicAuthConfig;
use self::compression::CompressionConfig;
use self::cors::CorsConfig;
use self::file::ConfigFile;
use self::proxy::ProxyConfig;
use self::tls::TlsConfig;

/// Server instance configuration used on initialization
pub struct Config {
    pub address: SocketAddr,
    pub host: IpAddr,
    pub port: u16,
    pub index: bool,
    pub spa: bool,
    pub root_dir: PathBuf,
    pub quiet: bool,
    pub tls: Option<TlsConfig>,
    pub cors: Option<CorsConfig>,
    pub compression: Option<CompressionConfig>,
    pub basic_auth: Option<BasicAuthConfig>,
    pub logger: Option<bool>,
    pub proxy: Option<ProxyConfig>,
    pub graceful_shutdown: bool,
}

impl Default for Config {
    fn default() -> Self {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(host, port);
        let root_dir = current_dir().unwrap_or_default();

        Self {
            host,
            port,
            index: false,
            spa: false,
            address,
            root_dir,
            quiet: false,
            tls: None,
            cors: None,
            compression: None,
            basic_auth: None,
            logger: None,
            proxy: None,
            graceful_shutdown: false,
        }
    }
}

impl TryFrom<Cli> for Config {
    type Error = Report;

    fn try_from(cli_arguments: Cli) -> Result<Self, Self::Error> {
        let root_dir = cli_arguments.root_dir.canonicalize()?;

        let tls: Option<TlsConfig> = if cli_arguments.tls {
            Some(TlsConfig::new(
                cli_arguments.tls_cert,
                cli_arguments.tls_key,
                cli_arguments.tls_key_algorithm,
            )?)
        } else {
            None
        };

        let cors: Option<CorsConfig> = if cli_arguments.cors {
            // when CORS is specified from CLI the default
            // configuration should allow any origin, method and
            // headers
            Some(CorsConfig::allow_all())
        } else {
            None
        };

        let compression: Option<CompressionConfig> = if cli_arguments.gzip {
            Some(CompressionConfig { gzip: true })
        } else {
            None
        };

        let basic_auth: Option<BasicAuthConfig> = if let (Some(username), Some(password)) =
            (cli_arguments.username, cli_arguments.password)
        {
            Some(BasicAuthConfig::new(username, password))
        } else {
            None
        };

        let logger = if cli_arguments.logger {
            Some(true)
        } else {
            None
        };

        let proxy = match cli_arguments.proxy {
            Some(proxy_url) => Some(ProxyConfig::url(Uri::from_str(&proxy_url)?)),
            None => None,
        };

        let spa = cli_arguments.spa;
        let index = spa || cli_arguments.index;

        Ok(Config {
            host: cli_arguments.host,
            port: cli_arguments.port,
            address: SocketAddr::new(cli_arguments.host, cli_arguments.port),
            index,
            spa,
            root_dir,
            quiet: cli_arguments.quiet,
            tls,
            cors,
            compression,
            basic_auth,
            logger,
            proxy,
            graceful_shutdown: cli_arguments.graceful_shutdown,
        })
    }
}

impl TryFrom<ConfigFile> for Config {
    type Error = Report;

    fn try_from(file: ConfigFile) -> Result<Self, Self::Error> {
        let root_dir = file.root_dir.unwrap_or_default();
        let quiet = file.quiet.unwrap_or(false);
        let tls: Option<TlsConfig> = if let Some(https_config) = file.tls {
            Some(TlsConfig::new(
                https_config.cert,
                https_config.key,
                https_config.key_algorithm,
            )?)
        } else {
            None
        };

        let spa = file.spa.unwrap_or(false);
        let index = spa || file.index.unwrap_or(false);

        Ok(Config {
            host: file.host,
            port: file.port,
            address: SocketAddr::new(file.host, file.port),
            index,
            spa,
            quiet,
            root_dir,
            tls,
            cors: file.cors,
            compression: file.compression,
            basic_auth: file.basic_auth,
            logger: file.logger,
            proxy: file.proxy,
            graceful_shutdown: file.graceful_shutdown.unwrap_or(false),
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
        assert!(!config.quiet, "quiet is off by default");
    }
}
