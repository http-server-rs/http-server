use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use http::request::Parts;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;
use tokio::runtime::Handle;

use http_server_plugin::{export_plugin, Plugin, PluginError, PluginRegistrar};

export_plugin!(register);

const PLUGIN_NAME: &str = "hello-world";

#[allow(improper_ctypes_definitions)]
extern "C" fn register(_: PathBuf, rt: Arc<Handle>, registrar: &mut dyn PluginRegistrar) {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    registrar.register_function(
        PLUGIN_NAME,
        Arc::new(HelloWorldPlugin::new(rt)),
    );
}

struct HelloWorldPlugin {
    rt: Arc<Handle>,
}

#[async_trait]
impl Plugin for HelloWorldPlugin {
    #[no_mangle]
    fn call(&self, parts: Parts, body: Bytes) -> Result<Response<Full<Bytes>>, PluginError> {
        println!("Test block on:");
        self.handle(parts, body)
    }
}

impl HelloWorldPlugin {
    fn new(rt: Arc<Handle>) -> Self {
        Self {
            rt,
        }
    }

    fn handle(
        &self,
        _: Parts,
        _: Bytes,
    ) -> Result<Response<Full<Bytes>>, PluginError> {
        Ok(Response::new(Full::new(Bytes::from("Hello World"))))
    }
}
