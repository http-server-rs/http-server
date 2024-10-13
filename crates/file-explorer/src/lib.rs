mod fs;
mod templater;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response};
use serde::Deserialize;
use tokio::runtime::Runtime;

use http_server_plugin::config::read_from_path;
use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

use self::fs::FileSystem;
use self::templater::Templater;

export_plugin!(register);

const PLUGIN_NAME: &str = "file-explorer";

#[allow(improper_ctypes_definitions)]
extern "C" fn register(
    config_path: PathBuf,
    rt: Arc<Runtime>,
    registrar: &mut dyn PluginRegistrar,
) {
    let config: FileExplorerConfig = read_from_path(config_path, PLUGIN_NAME).unwrap();

    registrar.register_function(
        PLUGIN_NAME,
        Arc::new(FileExplorer::new(rt, config.path).expect("Failed to create FileExplorer")),
    );
}

#[derive(Debug, Deserialize)]
struct FileExplorerConfig {
    pub path: PathBuf,
}

struct FileExplorer {
    rt: Arc<Runtime>,
    fs: FileSystem,
    path: PathBuf,
    templater: Templater,
}

#[async_trait]
impl Function for FileExplorer {
    async fn call(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.rt.block_on(async move {
            match req.method() {
                &Method::GET => {
                    match self
                        .fs
                        .resolve(PathBuf::new())
                        .await
                        .expect("failed to execute")
                    {
                        fs::Entry::File(file) => {
                            Ok(Response::new(Full::new(Bytes::from(file.bytes()))))
                        }
                        fs::Entry::Directory(dir) => {
                            Ok(Response::new(Full::new(Bytes::from(format!("{:?}", dir)))))
                        }
                    }
                }
                &Method::POST => Ok(Response::new(Full::new(Bytes::from("Prepare to upload")))),
                _ => Ok(Response::new(Full::new(Bytes::from("Unsupported method")))),
            }
        })
    }
}

impl FileExplorer {
    fn new(rt: Arc<Runtime>, path: PathBuf) -> Result<Self> {
        let fs = FileSystem::new(path.clone())?;
        let templater = Templater::new()?;

        Ok(Self {
            rt,
            fs,
            path,
            templater,
        })
    }
}
