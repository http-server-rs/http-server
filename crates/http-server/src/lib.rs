pub mod server;

use std::path::PathBuf;

use anyhow::{bail, Result};
use dirs::home_dir;

pub const HTTP_SERVER_HOME_DIRNAME: &str = ".http-server-rs";
pub const HTTP_SERVER_PLUGINS_DIRNAME: &str = "plugins";
pub const DEFAULT_PLUGIN_NAME: &str = "file_explorer.plugin.httprs";
pub const FILE_EXPLORER_PLUGIN_BYTES: &[u8] =
    include_bytes!("../../../target/debug/libfile_explorer_plugin.dylib");

pub fn install_path() -> Result<PathBuf> {
    let Some(home) = home_dir() else {
        bail!("Failed to resolve Home Directory. Unable to install.");
    };

    Ok(home.join(HTTP_SERVER_HOME_DIRNAME))
}

pub fn plugins_path() -> Result<PathBuf> {
    let install_path = install_path()?;
    let plugins_path = install_path.join(HTTP_SERVER_PLUGINS_DIRNAME);

    Ok(plugins_path)
}
