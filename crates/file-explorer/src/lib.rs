mod templater;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response};
use serde::Deserialize;

use http_server_plugin::config::read_from_path;
use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

use self::templater::Templater;

export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(config_path: PathBuf, registrar: &mut dyn PluginRegistrar) {
    let config: FileExplorerConfig = read_from_path(config_path, "file-explorer").unwrap();

    registrar.register_function(
        "file-explorer",
        Arc::new(FileExplorer::new(config.path).expect("Failed to create FileExplorer")),
    );
}

#[derive(Debug, Deserialize)]
struct FileExplorerConfig {
    pub path: PathBuf,
}

struct FileExplorer {
    pub path: PathBuf,
    pub templater: Templater,
}

impl Function for FileExplorer {
    fn call(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, InvocationError> {
        match req.method() {
            &Method::GET => Ok(Response::new(Full::new(Bytes::from("File Explorer")))),
            &Method::POST => Ok(Response::new(Full::new(Bytes::from("Prepare to upload")))),
            _ => Ok(Response::new(Full::new(Bytes::from("Unsupported method")))),
        }
    }
}

impl FileExplorer {
    fn new(path: PathBuf) -> Result<Self> {
        let templater = Templater::new()?;

        Ok(Self { path, templater })
    }
}
