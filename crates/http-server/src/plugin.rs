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
use tokio::sync::Mutex;

use http_server_plugin::{
    Function, InvocationError, PluginDeclaration, CORE_VERSION, RUSTC_VERSION,
};

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
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

#[derive(Default)]
pub struct ExternalFunctions {
    functions: Mutex<HashMap<String, FunctionProxy>>,
    libraries: Mutex<Vec<Arc<Library>>>,
}

impl ExternalFunctions {
    pub fn new() -> ExternalFunctions {
        ExternalFunctions::default()
    }

    /// Loads a plugin from the given path.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it loads a shared library and calls
    /// functions from it.
    pub async unsafe fn load<P: AsRef<OsStr>>(
        &self,
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

        (decl.register)(config_path, &mut registrar);

        self.functions.lock().await.extend(registrar.functions);

        self.libraries.lock().await.push(library);

        Ok(())
    }

    pub async fn call(
        &self,
        function: &str,
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.functions
            .lock()
            .await
            .get(function)
            .ok_or_else(|| format!("\"{}\" not found", function))
            .unwrap()
            .call(req)
            .await
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
