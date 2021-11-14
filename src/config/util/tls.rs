use anyhow::{ensure, Context, Error, Result};
use rustls::internal::msgs::codec::{Codec, Reader};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{pkcs8_private_keys, rsa_private_keys};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

/// Load certificate on the provided `path` and retrieve it
/// as an instance of `Vec<Certificate>`.
pub fn load_cert(path: &Path) -> Result<Vec<Certificate>> {
    let file = File::open(path.to_path_buf()).context(format!(
        "Unable to find the TLS certificate on: {}",
        path.to_str().unwrap()
    ))?;
    let mut buf_reader = BufReader::new(file);
    let bytes = buf_reader
        .fill_buf()
        .context("Failed to read cerficate bytes.")?;

    ensure!(bytes.len() == 0, "The provided certificate is empty");

    let mut reader = Reader::init(bytes);

    if let Some(cert) = Certificate::read(&mut reader) {
        return Ok(vec![cert]);
    }

    Err(Error::msg("Failed to read certificate"))
}

pub fn load_private_key(path: &Path, kind: &PrivateKeyAlgorithm) -> Result<PrivateKey> {
    let file = File::open(path.to_path_buf())
        .with_context(|| format!("Unable to find the TLS keys on: {}", path.to_str().unwrap()))?;
    let mut reader = BufReader::new(file);
    let keys = match kind {
        PrivateKeyAlgorithm::Rsa => rsa_private_keys(&mut reader).map_err(|_| {
            let path = path.to_str().unwrap();

            Error::msg(format!("Failed to read private (RSA) keys at {}", path))
        })?,
        PrivateKeyAlgorithm::Pkcs8 => pkcs8_private_keys(&mut reader).map_err(|_| {
            let path = path.to_str().unwrap();

            Error::msg(format!("Failed to read private (PKCS8) keys at {}", path))
        })?,
    };

    ensure!(keys.len() == 1, "Expected a single private key");
    Ok(PrivateKey(keys[0].clone()))
}
