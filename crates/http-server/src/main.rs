pub mod cli;
pub mod config;
pub mod plugin;
pub mod server;

use std::{path::PathBuf, process::exit, str::FromStr};

use anyhow::Result;

use crate::plugin::ExternalFunctions;

use self::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let mut functions = ExternalFunctions::new();
    let plugin_library = PathBuf::from_str("./target/debug/libfile_explorer.dylib").unwrap();

    unsafe {
        functions
            .load(plugin_library)
            .expect("Function loading failed");
    }

    let result = functions
        .call("file-explorer", &[])
        .expect("Invocation failed");

    println!("file-explorer() = {}", result);

    match Server::run().await {
        Ok(_) => {
            println!("Server exited successfuly");
            Ok(())
        }
        Err(error) => {
            eprint!("{:?}", error);
            exit(1);
        }
    }
}
