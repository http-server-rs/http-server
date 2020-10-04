use crate::cli::{ADDRESS, PORT, ROOT_DIR, SILENT};
use clap::App;
use std::env::current_dir;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

/// Configuration for the HTTP/S Server
#[derive(Debug)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub socket_address: SocketAddr,
    pub root_dir: PathBuf,
    pub silent: bool,
}

impl From<App<'static, 'static>> for Config {
    fn from(app: App) -> Self {
        let matches = app.get_matches();
        let address = IpAddr::from_str(matches.value_of(ADDRESS.1).unwrap()).unwrap();
        let port = matches.value_of(PORT.1).unwrap().parse::<u16>().unwrap();
        let socket_address = SocketAddr::new(address, port);
        let root_dir = if let Some(root_dir) = matches.value_of(ROOT_DIR.1) {
            PathBuf::from_str(root_dir).unwrap()
        } else {
            current_dir().unwrap()
        };

        let silent = matches.is_present(SILENT.1);

        // at this point the values provided to the config are validated by the CLI
        Self {
            address,
            port,
            socket_address,
            root_dir,
            silent,
        }
    }
}
