use color_eyre::eyre::Context;
use rustls::{Certificate, PrivateKey};
use serde::Deserialize;
use std::path::PathBuf;

use super::util::tls::{load_cert, load_private_key, PrivateKeyAlgorithm};

/// Configuration for TLS protocol serving with its certificate and private key
#[derive(Clone, Debug)]
pub struct TlsConfig {
    cert: Vec<Certificate>,
    key: PrivateKey,
    #[allow(dead_code)]
    key_algorithm: PrivateKeyAlgorithm,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TlsConfigFile {
    pub cert: PathBuf,
    pub key: PathBuf,
    pub key_algorithm: PrivateKeyAlgorithm,
}

impl TlsConfig {
    pub fn new(
        cert_path: PathBuf,
        key_path: PathBuf,
        key_algorithm: PrivateKeyAlgorithm,
    ) -> color_eyre::Result<Self> {
        let cert = load_cert(&cert_path).context("Failed to load certificate")?;
        let key =
            load_private_key(&key_path, &key_algorithm).context("Failed to load private key")?;

        Ok(TlsConfig {
            cert,
            key,
            key_algorithm,
        })
    }

    /// Retrieve certificates and private key loaded on initialization
    pub fn parts(&self) -> (Vec<Certificate>, PrivateKey) {
        (self.cert.clone(), self.key.clone())
    }
}
