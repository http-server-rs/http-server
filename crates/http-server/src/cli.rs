use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The address to listen on
    #[arg(long, default_value = "127.0.0.1")]
    pub host: IpAddr,
    /// The port to listen on
    #[arg(long, default_value = "7878")]
    pub port: u16,
    /// Root directory for File Explorer
    #[arg(long, default_value = "./")]
    pub root_dir: PathBuf,
}

impl Cli {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
