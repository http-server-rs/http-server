use color_eyre::eyre::bail;
use color_eyre::eyre::eyre;
use color_eyre::eyre::Context;
use color_eyre::Report;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{pkcs8_private_keys, rsa_private_keys};
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
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rsa" => Ok(PrivateKeyAlgorithm::Rsa),
            "pkcs8" => Ok(PrivateKeyAlgorithm::Pkcs8),
            _ => bail!("Invalid algorithm name provided for TLS key. {}", s),
        }
    }
}

/// Load certificate on the provided `path` and retrieve it
/// as an instance of `Vec<Certificate>`.
pub fn load_cert(path: &Path) -> color_eyre::Result<Vec<Certificate>> {
    let file =
        File::open(path).context(format!("Unable to find the TLS certificate on: {path:?}",))?;
    let mut buf_reader = BufReader::new(file);
    let cert_bytes = &rustls_pemfile::certs(&mut buf_reader)?[0];

    Ok(vec![Certificate(cert_bytes.to_vec())])
}

pub fn load_private_key(path: &Path, kind: &PrivateKeyAlgorithm) -> color_eyre::Result<PrivateKey> {
    let file =
        File::open(path).with_context(|| format!("Unable to find the TLS keys on: {path:?}"))?;
    let mut reader = BufReader::new(file);
    let keys = match kind {
        PrivateKeyAlgorithm::Rsa => rsa_private_keys(&mut reader)
            .map_err(|_| eyre!("Failed to read private (RSA) keys at {path:?}"))?,
        PrivateKeyAlgorithm::Pkcs8 => pkcs8_private_keys(&mut reader)
            .map_err(|_| eyre!("Failed to read private (PKCS8) keys at {path:?}"))?,
    };

    Ok(PrivateKey(keys[0].clone()))
}
