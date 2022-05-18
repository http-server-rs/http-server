use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

use crate::config::util::tls::PrivateKeyAlgorithm;

#[derive(Debug, StructOpt, PartialEq, Eq)]
#[structopt(
    name = "http-server",
    author = "Esteban Borai <estebanborai@gmail.com>",
    about = "Simple and configurable command-line HTTP server\nSource: https://github.com/EstebanBorai/http-server"
)]
pub struct Cli {
    /// Path to TOML configuration file.
    #[structopt(parse(from_os_str), short = "c", long = "config")]
    pub config: Option<PathBuf>,
    /// Host (IP) to bind the server
    #[structopt(short = "h", long = "host", default_value = "127.0.0.1")]
    pub host: IpAddr,
    /// Port to bind the server
    #[structopt(short = "p", long = "port", default_value = "7878")]
    pub port: u16,
    /// Directory to serve files from
    #[structopt(parse(from_os_str), default_value = "./")]
    pub root_dir: PathBuf,
    /// Turns on stdout/stderr logging
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    /// Enables HTTPS serving using TLS
    #[structopt(long = "tls")]
    pub tls: bool,
    /// Path to the TLS Certificate
    #[structopt(long = "tls-cert", parse(from_os_str), default_value = "cert.pem")]
    pub tls_cert: PathBuf,
    /// Path to the TLS Key
    #[structopt(long = "tls-key", parse(from_os_str), default_value = "key.rsa")]
    pub tls_key: PathBuf,
    /// Algorithm used to generate certificate key
    #[structopt(long = "tls-key-algorithm", default_value = "rsa")]
    pub tls_key_algorithm: PrivateKeyAlgorithm,
    /// Enable Cross-Origin Resource Sharing allowing any origin
    #[structopt(long = "cors")]
    pub cors: bool,
    /// Enable GZip compression for HTTP Responses
    #[structopt(long = "gzip")]
    pub gzip: bool,
    /// Specifies username for basic authentication
    #[structopt(long = "username")]
    pub username: Option<String>,
    /// Specifies password for basic authentication
    #[structopt(long = "password")]
    pub password: Option<String>,
    /// Prints HTTP request and response details to stdout
    #[structopt(long = "logger")]
    pub logger: bool,
    /// Proxy requests to the provided URL
    #[structopt(long = "proxy")]
    pub proxy: Option<String>,
    /// Waits for all requests to fulfill before shutting down the server
    #[structopt(long = "graceful-shutdown")]
    pub graceful_shutdown: bool,
}

impl Cli {
    pub fn from_str_args(args: Vec<&str>) -> Self {
        Cli::from_iter_safe(args.into_iter()).unwrap_or_else(|e| e.exit())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            config: None,
            host: "127.0.0.1".parse().unwrap(),
            port: 7878_u16,
            root_dir: PathBuf::from_str("./").unwrap(),
            verbose: false,
            tls: false,
            tls_cert: PathBuf::from_str("cert.pem").unwrap(),
            tls_key: PathBuf::from_str("key.rsa").unwrap(),
            tls_key_algorithm: PrivateKeyAlgorithm::Rsa,
            cors: false,
            gzip: false,
            username: None,
            password: None,
            logger: false,
            proxy: None,
            graceful_shutdown: false,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn no_arguments() {
        let from_args = Cli::from_str_args(vec![]);
        let expect = Cli::default();

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_host() {
        let from_args = Cli::from_str_args(vec!["http-server", "--host", "0.0.0.0"]);
        let mut expect = Cli::default();

        expect.host = "0.0.0.0".parse().unwrap();

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_host_and_port() {
        let from_args = Cli::from_str_args(vec![
            "http-server",
            "--host",
            "192.168.0.1",
            "--port",
            "54200",
        ]);
        let mut expect = Cli::default();

        expect.host = "192.168.0.1".parse().unwrap();
        expect.port = 54200 as u16;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_root_dir() {
        let from_args = Cli::from_str_args(vec!["http-server", "~/User/sources/http-server"]);
        let mut expect = Cli::default();

        expect.root_dir = PathBuf::from_str("~/User/sources/http-server").unwrap();

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_verbose() {
        let from_args = Cli::from_str_args(vec!["http-server", "--verbose"]);
        let mut expect = Cli::default();

        expect.verbose = true;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_tls_no_config() {
        let from_args = Cli::from_str_args(vec!["http-server", "--tls"]);
        let mut expect = Cli::default();

        expect.tls = true;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_tls_and_config() {
        let from_args = Cli::from_str_args(vec![
            "http-server",
            "--tls",
            "--tls-cert",
            "~/secrets/cert",
            "--tls-key",
            "~/secrets/key",
            "--tls-key-algorithm",
            "rsa",
        ]);
        let mut expect = Cli::default();

        expect.tls = true;
        expect.tls_cert = PathBuf::from_str("~/secrets/cert").unwrap();
        expect.tls_key = PathBuf::from_str("~/secrets/key").unwrap();
        expect.tls_key_algorithm = PrivateKeyAlgorithm::Rsa;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_cors() {
        let from_args = Cli::from_str_args(vec!["http-server", "--cors"]);
        let mut expect = Cli::default();

        expect.cors = true;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_gzip() {
        let from_args = Cli::from_str_args(vec!["http-server", "--gzip"]);
        let mut expect = Cli::default();

        expect.gzip = true;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_basic_auth() {
        let from_args = Cli::from_str_args(vec![
            "http-server",
            "--username",
            "John",
            "--password",
            "Appleseed",
        ]);
        let mut expect = Cli::default();

        expect.username = Some(String::from("John"));
        expect.password = Some(String::from("Appleseed"));

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_username_but_not_password() {
        let from_args = Cli::from_str_args(vec!["http-server", "--username", "John"]);
        let mut expect = Cli::default();

        expect.username = Some(String::from("John"));
        expect.password = None;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_password_but_not_username() {
        let from_args = Cli::from_str_args(vec!["http-server", "--password", "Appleseed"]);
        let mut expect = Cli::default();

        expect.username = None;
        expect.password = Some(String::from("Appleseed"));

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_logger() {
        let from_args = Cli::from_str_args(vec!["http-server", "--logger"]);
        let mut expect = Cli::default();

        expect.logger = true;

        assert_eq!(from_args, expect);
    }

    #[test]
    fn with_proxy() {
        let from_args = Cli::from_str_args(vec!["http-server", "--proxy", "https://example.com"]);
        let mut expect = Cli::default();

        expect.proxy = Some(String::from("https://example.com"));

        assert_eq!(from_args, expect);
    }
}
