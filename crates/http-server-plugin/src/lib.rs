pub mod config;

use std::path::PathBuf;
use std::sync::Arc;

use http::request::Parts;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;
use tokio::runtime::Handle;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Plugin: Send + Sync {
    fn call(&self, parts: Parts, body: Bytes) -> Result<Response<Full<Bytes>>, PluginError>;
}

#[derive(Debug)]
pub enum PluginError {
    SpawnError { err: String },
    Other { msg: String },
}

#[allow(improper_ctypes_definitions)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register:
        unsafe extern "C" fn(config_path: PathBuf, rt: Arc<Handle>, &mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Arc<dyn Plugin>);
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PLUGIN_DECLARATION: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}
