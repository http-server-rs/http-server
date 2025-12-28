use std::net::IpAddr;
use std::str::FromStr;

use anyhow::{Error, Result, bail};

#[derive(Clone, Debug)]
pub enum Service {
    FileServer {
        root_directory: String,
        basic_auth: Option<BasicAuth>,
    },
    FileExplorer {
        root_directory: String,
        basic_auth: Option<BasicAuth>,
    },
}

impl From<crate::cli::command::start::Service> for Service {
    fn from(val: crate::cli::command::start::Service) -> Self {
        match val {
            crate::cli::command::start::Service::FileServer => Service::FileServer {
                root_directory: "./".into(),
                basic_auth: None,
            },
            crate::cli::command::start::Service::FileExplorer => Service::FileExplorer {
                root_directory: "./".into(),
                basic_auth: None,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    /// The IP address to bind to.
    pub host: IpAddr,
    /// The port to bind to.
    pub port: u16,
    /// Enable CORS with a permissive policy.
    pub cors: bool,
    /// Service
    pub service: Service,
}

#[derive(Clone, Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl FromStr for BasicAuth {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(":").collect::<Vec<&str>>();

        if parts.len() != 2 {
            bail!(
                "Expected a string with a colon to separe username and password for Basic Authentication."
            );
        }

        Ok(BasicAuth {
            username: parts[0].into(),
            password: parts[1].into(),
        })
    }
}
