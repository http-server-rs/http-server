use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::config::util::tls::PrivateKeyAlgorithm;

#[derive(Debug, StructOpt)]
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
}
