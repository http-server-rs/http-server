use anyhow::{Context, Error, Result};
use rustls::internal::pemfile;
use rustls::{Certificate, PrivateKey};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_cert(path: &PathBuf) -> Result<Vec<Certificate>> {
    let file = File::open(path.clone()).context(format!(
        "Unable to find the TLS certificate on: {}",
        path.to_str().unwrap()
    ))?;
    let mut reader = BufReader::new(file);

    pemfile::certs(&mut reader).map_err(|_| {
        let path = path.to_str().unwrap();

        Error::msg(format!("Failed to read certificates at {}", path))
    })
}

pub fn load_private_key(path: &PathBuf) -> Result<PrivateKey> {
    let file = File::open(path.clone()).context(format!(
        "Unable to find the TLS keys on: {}",
        path.to_str().unwrap()
    ))?;
    let mut reader = BufReader::new(file);
    let keys = pemfile::rsa_private_keys(&mut reader).map_err(|_| {
        let path = path.to_str().unwrap();

        Error::msg(format!("Failed to read private keys at {}", path))
    })?;

    if keys.len() != 1 {
        return Err(Error::msg("Expected a single private key"));
    }

    Ok(keys[0].clone())
}
