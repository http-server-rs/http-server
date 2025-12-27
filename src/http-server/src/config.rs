use std::net::IpAddr;
use std::str::FromStr;

use anyhow::{Error, Result, bail};

#[derive(Clone, Debug)]
pub struct Config {
    /// The IP address to bind to.
    pub host: IpAddr,
    /// The port to bind to.
    pub port: u16,
    /// Enable CORS with a permissive policy.
    pub cors: bool,
    /// Enable Legacy File Explorer UI.
    pub legacy_ui: bool,
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
