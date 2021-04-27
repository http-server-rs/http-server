use anyhow::{ensure, Context, Error, Result};
use rustls::internal::pemfile;
use rustls::{Certificate, PrivateKey};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum PrivateKeyAlgorithm {
    #[serde(rename = "rsa")]
    Rsa,
    #[serde(rename = "pkcs8")]
    Pkcs8,
}

impl FromStr for PrivateKeyAlgorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rsa" => Ok(PrivateKeyAlgorithm::Rsa),
            "pkcs8" => Ok(PrivateKeyAlgorithm::Pkcs8),
            _ => anyhow::bail!("Invalid algorithm name provided for TLS key. {}", s),
        }
    }
}

pub fn load_cert(path: &Path) -> Result<Vec<Certificate>> {
    let file = File::open(path.to_path_buf()).context(format!(
        "Unable to find the TLS certificate on: {}",
        path.to_str().unwrap()
    ))?;
    let mut reader = BufReader::new(file);

    pemfile::certs(&mut reader).map_err(|_| {
        let path = path.to_str().unwrap();

        Error::msg(format!("Failed to read certificates at {}", path))
    })
}

pub fn load_private_key(path: &Path, kind: &PrivateKeyAlgorithm) -> Result<PrivateKey> {
    let file = File::open(path.to_path_buf())
        .with_context(|| format!("Unable to find the TLS keys on: {}", path.to_str().unwrap()))?;
    let mut reader = BufReader::new(file);
    let keys = match kind {
        PrivateKeyAlgorithm::Rsa => pemfile::rsa_private_keys(&mut reader).map_err(|_| {
            let path = path.to_str().unwrap();

            Error::msg(format!("Failed to read private (RSA) keys at {}", path))
        })?,
        PrivateKeyAlgorithm::Pkcs8 => pemfile::pkcs8_private_keys(&mut reader).map_err(|_| {
            let path = path.to_str().unwrap();

            Error::msg(format!("Failed to read private (PKCS8) keys at {}", path))
        })?,
    };

    ensure!(keys.len() == 1, "Expected a single private key");

    Ok(keys[0].clone())
}
