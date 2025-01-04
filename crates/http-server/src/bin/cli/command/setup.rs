use std::fs::{create_dir_all, write};

use anyhow::Result;
use clap::Parser;

use http_server_rs::{install_path, plugins_path, DEFAULT_PLUGIN_NAME, FILE_EXPLORER_PLUGIN_BYTES};

#[derive(Debug, Parser)]
pub struct SetupOpt {}

impl SetupOpt {
    pub fn exec(&self) -> Result<()> {
        let http_server_install_path = install_path()?;

        if http_server_install_path.exists() {
            println!("HTTP Server System Files already installed.");
        }

        let plugins_path = plugins_path()?;

        if !plugins_path.exists() {
            println!(
                "Creating HTTP Server System Files at: {}",
                http_server_install_path.display()
            );
            create_dir_all(&plugins_path)?;
        }

        let file_explorer_plugin_path = plugins_path.join(DEFAULT_PLUGIN_NAME);

        if !file_explorer_plugin_path.exists() {
            println!(
                "Installing File Explorer Plugin at: {}",
                file_explorer_plugin_path.display()
            );
            write(&file_explorer_plugin_path, FILE_EXPLORER_PLUGIN_BYTES)?;
        }

        println!("HTTP Server Installed");
        Ok(())
    }
}
