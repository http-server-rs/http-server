use crate::cli::{ADDRESS, PORT, SILENT};
use clap::App;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

/// Configuration for the HTTP/S Server
#[derive(Debug)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub socket_address: SocketAddr,
    pub silent: bool,
    // pub input: PathBuf,
}

impl From<App<'static, 'static>> for Config {
    fn from(app: App) -> Self {
        let matches = app.get_matches();
        let address = IpAddr::from_str(matches.value_of(ADDRESS.1).unwrap()).unwrap();
        let port = matches.value_of(PORT.1).unwrap().parse::<u16>().unwrap();
        let socket_address = SocketAddr::new(address, port);
        let silent = matches.is_present(SILENT.1);

        // at this point the values provided to the config are validated by the CLI
        Self {
            address,
            port,
            socket_address,
            silent,
        }
    }
}
