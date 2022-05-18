use anyhow::{Error, Result};
use serde::{Deserialize, Deserializer};
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

use super::basic_auth::BasicAuthConfig;
use super::compression::CompressionConfig;
use super::cors::CorsConfig;
use super::proxy::ProxyConfig;
use super::tls::TlsConfigFile;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub host: IpAddr,
    pub port: u16,
    pub verbose: Option<bool>,
    #[serde(default = "current_working_dir")]
    #[serde(deserialize_with = "canonicalize_some")]
    pub root_dir: Option<PathBuf>,
    pub tls: Option<TlsConfigFile>,
    pub cors: Option<CorsConfig>,
    pub compression: Option<CompressionConfig>,
    pub basic_auth: Option<BasicAuthConfig>,
    pub logger: Option<bool>,
    pub proxy: Option<ProxyConfig>,
    pub graceful_shutdown: bool,
}

impl ConfigFile {
    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        let file = fs::read_to_string(file_path)?;
        let config = ConfigFile::parse_toml(file.as_str())?;

        Ok(config)
    }

    fn parse_toml(content: &str) -> Result<Self> {
        match toml::from_str(content) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::msg(format!(
                "Failed to parse config from file. {}",
                err
            ))),
        }
    }
}

fn canonicalize_some<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    let path = PathBuf::from_str(value).unwrap();
    let canon = fs::canonicalize(path).unwrap();

    Ok(Some(canon))
}

fn current_working_dir() -> Option<PathBuf> {
    std::env::current_dir().ok()
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::str::FromStr;

    use crate::config::util::tls::PrivateKeyAlgorithm;

    use super::*;

    #[test]
    fn parses_config_from_file() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878
            verbose = true
            root_dir = "./fixtures"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let config = ConfigFile::parse_toml(file_contents).unwrap();
        let mut root_dir = PathBuf::from(std::env::current_dir().unwrap());

        root_dir.push("./fixtures");

        assert!(!config.graceful_shutdown);
        assert!(config.logger.is_none());
        assert!(config.compression.is_none());
        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.verbose, Some(true));
        assert_eq!(config.root_dir, Some(root_dir));
    }

    #[test]
    #[should_panic(
        expected = "Failed to parse config from file. missing field `host` at line 1 column 1"
    )]
    fn checks_invalid_config_from_file() {
        let file_contents = r#"
            port = 7878
        "#;
        ConfigFile::parse_toml(file_contents).unwrap();
    }

    #[test]
    fn parses_config_with_tls_using_rsa() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878
            verbose = false

            [tls]
            cert = "cert_123.pem"
            key = "key_123.pem"
            key_algorithm = "rsa"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let root_dir = Some(std::env::current_dir().unwrap());
        let tls = TlsConfigFile {
            cert: PathBuf::from_str("cert_123.pem").unwrap(),
            key: PathBuf::from_str("key_123.pem").unwrap(),
            key_algorithm: PrivateKeyAlgorithm::Rsa,
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir, root_dir);
        assert_eq!(config.tls.unwrap(), tls);
        assert_eq!(config.verbose, Some(false));
    }

    #[test]
    fn parses_config_with_tls_using_pkcs8() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878

            [tls]
            cert = "cert_123.pem"
            key = "key_123.pem"
            key_algorithm = "pkcs8"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let root_dir = Some(std::env::current_dir().unwrap());
        let tls = TlsConfigFile {
            cert: PathBuf::from_str("cert_123.pem").unwrap(),
            key: PathBuf::from_str("key_123.pem").unwrap(),
            key_algorithm: PrivateKeyAlgorithm::Pkcs8,
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir, root_dir);
        assert_eq!(config.tls.unwrap(), tls);
    }

    #[test]
    fn parses_basic_cors_config_from_file() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 8080

            [cors]
            allow_credentials = true
            allow_headers = ["content-type", "authorization", "content-length"]
            allow_methods = ["GET", "PATCH", "POST", "PUT", "DELETE"]
            allow_origin = "example.com"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let port = 8080;
        let cors = CorsConfig {
            allow_credentials: true,
            allow_headers: Some(vec![
                "content-type".to_string(),
                "authorization".to_string(),
                "content-length".to_string(),
            ]),
            allow_methods: Some(vec![
                "GET".to_string(),
                "PATCH".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ]),
            allow_origin: Some(String::from("example.com")),
            expose_headers: None,
            max_age: None,
            request_headers: None,
            request_method: None,
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();
        let root_dir = Some(std::env::current_dir().unwrap());

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir, root_dir);
        assert_eq!(config.cors.unwrap(), cors);
    }

    #[test]
    fn parses_complex_cors_config_from_file() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 8080

            [cors]
            allow_credentials = true
            allow_headers = ["content-type", "authorization", "content-length"]
            allow_methods = ["GET", "PATCH", "POST", "PUT", "DELETE"]
            allow_origin = "example.com"
            expose_headers = ["*", "authorization"]
            max_age = 2800
            request_headers = ["x-app-version"]
            request_method = "GET"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let port = 8080;
        let root_dir = Some(std::env::current_dir().unwrap());
        let cors = CorsConfig {
            allow_credentials: true,
            allow_headers: Some(vec![
                "content-type".to_string(),
                "authorization".to_string(),
                "content-length".to_string(),
            ]),
            allow_methods: Some(vec![
                "GET".to_string(),
                "PATCH".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ]),
            allow_origin: Some(String::from("example.com")),
            expose_headers: Some(vec!["*".to_string(), "authorization".to_string()]),
            max_age: Some(2800),
            request_headers: Some(vec!["x-app-version".to_string()]),
            request_method: Some(String::from("GET")),
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir, root_dir);
        assert_eq!(config.cors.unwrap(), cors);
    }

    #[test]
    fn parses_config_with_gzip_compression() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 7878

            [compression]
            gzip = true
        "#;
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert!(config.compression.unwrap().gzip);
    }

    #[test]
    fn parses_config_with_basic_auth() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 7878

            [basic_auth]
            username = "johnappleseed"
            password = "john::likes::apples!"
        "#;
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert!(config.basic_auth.is_some());

        let basic_auth = config.basic_auth.unwrap();

        assert_eq!(basic_auth.username, String::from("johnappleseed"));
        assert_eq!(basic_auth.password, String::from("john::likes::apples!"));
    }

    #[test]
    fn parses_config_with_logger() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 7878
            logger = true
        "#;
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.logger, Some(true));
    }

    #[test]
    fn parses_config_with_proxy() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 7878

            [proxy]
            url = "https://example.com"
        "#;
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert!(config.proxy.is_some());

        let proxy = config.proxy.unwrap();

        assert_eq!(proxy.url, "https://example.com");
    }

    #[test]
    fn parse_config_with_graceful_shutdown() {
        let file_contents = r#"
            host = "0.0.0.0"
            port = 7878
            graceful_shutdown = true
        "#;
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert!(config.graceful_shutdown);
    }
}
