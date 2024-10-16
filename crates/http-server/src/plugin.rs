use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use libloading::Library;
use tokio::runtime::Handle;
use tokio::sync::Mutex;

use http_server_plugin::{
    Function, InvocationError, PluginDeclaration, CORE_VERSION, RUSTC_VERSION,
};

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
#[derive(Clone)]
pub struct FunctionProxy {
    function: Arc<dyn Function>,
    _lib: Arc<Library>,
}

#[async_trait]
impl Function for FunctionProxy {
    async fn call(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.function.call(req).await
    }
}

pub struct ExternalFunctions {
    handle: Arc<Handle>,
    functions: Mutex<HashMap<String, FunctionProxy>>,
    libraries: Mutex<Vec<Arc<Library>>>,
}

impl Default for ExternalFunctions {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalFunctions {
    pub fn new() -> ExternalFunctions {
        let handle = Arc::new(Handle::current());

        ExternalFunctions {
            handle,
            functions: Mutex::new(HashMap::default()),
            libraries: Mutex::new(Vec::new()),
        }
    }

    /// Loads a plugin from the given path.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it loads a shared library and calls
    /// functions from it.
    pub async unsafe fn load<P: AsRef<OsStr>>(
        &self,
        rt_handle: Arc<Handle>,
        config_path: PathBuf,
        library_path: P,
    ) -> io::Result<()> {
        let library = Arc::new(Library::new(library_path).unwrap());
        let decl = library
            .get::<*mut PluginDeclaration>(b"PLUGIN_DECLARATION\0")
            .unwrap()
            .read();

        if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Arc::clone(&library));

        (decl.register)(config_path, Arc::clone(&rt_handle), &mut registrar);

        self.functions.lock().await.extend(registrar.functions);
        self.libraries.lock().await.push(library);

        Ok(())
    }

    async fn get_function(&self, func: &str) -> Option<FunctionProxy> {
        self.functions.lock().await.get(func).cloned()
    }

    pub async fn call(
        &self,
        func: &str,
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        let function_proxy = self.get_function(func).await.unwrap();
        let join_handle = self
            .handle
            .spawn(async move { function_proxy.call(req).await })
            .await;

        join_handle.unwrap()
    }
}

struct PluginRegistrar {
    functions: HashMap<String, FunctionProxy>,
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
    fn register_function(&mut self, name: &str, function: Arc<dyn Function>) {
        let proxy = FunctionProxy {
            function,
            _lib: Arc::clone(&self.lib),
        };

        self.functions.insert(name.to_string(), proxy);
    }
}
