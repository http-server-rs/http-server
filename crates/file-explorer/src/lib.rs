use std::sync::Arc;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};

use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function(
        "file-explorer",
        Arc::new(FileExplorer {
            path: String::from("/"),
        }),
    );
}

pub struct FileExplorer {
    pub path: String,
}

impl Function for FileExplorer {
    fn call(&self, _: Request<Incoming>) -> Result<Response<Full<Bytes>>, InvocationError> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }
}
