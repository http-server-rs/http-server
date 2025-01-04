use std::collections::HashMap;
use std::io::{Error as IOError, Result as IOResult};
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use http::request::Parts;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;
use libloading::Library;
use tokio::runtime::Handle;
use tokio::sync::Mutex;

use http_server_plugin::{Plugin, PluginDeclaration, PluginError, CORE_VERSION, RUSTC_VERSION};

use crate::plugins_path;

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
#[derive(Clone)]
pub struct PluginProxy {
    function: Arc<dyn Plugin>,
    _lib: Arc<Library>,
}

#[async_trait]
impl Plugin for PluginProxy {
    async fn call(&self, parts: Parts, bytes: Bytes) -> Result<Response<Full<Bytes>>, PluginError> {
        self.function.call(parts, bytes).await
    }
}

pub struct PluginStore {
    functions: Mutex<HashMap<String, PluginProxy>>,
    handle: Arc<Handle>,
    libraries: Mutex<Vec<Arc<Library>>>,
}

impl Default for PluginStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginStore {
    pub fn new() -> Self {
        let handle = Arc::new(Handle::current());

        Self {
            functions: Mutex::new(HashMap::default()),
            handle,
            libraries: Mutex::new(Vec::new()),
        }
    }

    /// Loads a plugin from the given path.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it loads a shared library and calls
    /// functions from it.
    pub async unsafe fn load(
        &self,
        rt_handle: Arc<Handle>,
        config_path: PathBuf,
        plugin_filename: &str,
    ) -> IOResult<()> {
        let plugin_path = plugins_path()
            .map_err(|err| IOError::other(format!("Failed to retrieve plugin path: {err}")))?
            .join(plugin_filename);
        let library = Library::new(&plugin_path).map_err(|err| {
            IOError::other(format!("Failed to load plugin from {plugin_path:?}: {err}"))
        })?;
        let library = Arc::new(library);
        let decl = library
            .get::<*mut PluginDeclaration>(b"PLUGIN_DECLARATION\0")
            .unwrap()
            .read();

        if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
            return Err(IOError::other("Version Mismatch."));
        }

        let mut registrar = PluginRegistrar::new(Arc::clone(&library));

        (decl.register)(config_path, Arc::clone(&rt_handle), &mut registrar);

        self.functions.lock().await.extend(registrar.functions);
        self.libraries.lock().await.push(library);

        Ok(())
    }

    async fn get(&self, func: &str) -> Option<PluginProxy> {
        self.functions.lock().await.get(func).cloned()
    }

    pub async fn run(
        &self,
        plugin: &str,
        parts: Parts,
        bytes: Bytes,
    ) -> Result<Response<Full<Bytes>>, PluginError> {
        let function_proxy = self.get(plugin).await.unwrap();
        let join_handle = self
            .handle
            .spawn(async move { function_proxy.call(parts, bytes).await })
            .await;

        join_handle.map_err(|err| PluginError::SpawnError {
            err: err.to_string(),
        })?
    }
}

struct PluginRegistrar {
    functions: HashMap<String, PluginProxy>,
    lib: Arc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Arc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            functions: HashMap::default(),
        }
    }
}

impl http_server_plugin::PluginRegistrar for PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Arc<dyn Plugin>) {
        let proxy = PluginProxy {
            function,
            _lib: Arc::clone(&self.lib),
        };

        self.functions.insert(name.to_string(), proxy);
    }
}
