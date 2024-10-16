use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::{bail, Result};
use serde::de::DeserializeOwned;
use toml::Table;

pub fn read_from_path<T: DeserializeOwned>(path: PathBuf, key: &str) -> Result<T> {
    let config_str = read_to_string(&path)?;
    let config_tbl: Table = toml::from_str(&config_str)?;

    if let Some(tbl) = config_tbl.get(key) {
        let config: T = tbl.to_owned().try_into().unwrap();
        return Ok(config);
    }

    bail!("Key not found")
}
