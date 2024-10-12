mod templater;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response};

use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

use self::templater::Templater;

export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function(
        "file-explorer",
        Arc::new(FileExplorer::new(PathBuf::new()).expect("Failed to create FileExplorer")),
    );
}

pub struct FileExplorer {
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
    pub fn new(path: PathBuf) -> Result<Self> {
        let templater = Templater::new()?;

        Ok(Self {
            path,
            templater,
        })
    }
}
