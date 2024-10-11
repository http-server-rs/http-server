use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::sync::{Arc, Mutex};

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use libloading::Library;

use http_server_plugin::{
    Function, InvocationError, PluginDeclaration, CORE_VERSION, RUSTC_VERSION,
};

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct FunctionProxy {
    function: Arc<dyn Function>,
    _lib: Arc<Library>,
}

impl Function for FunctionProxy {
    fn call(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.function.call(req)
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
    pub unsafe fn load<P: AsRef<OsStr>>(&self, library_path: P) -> io::Result<()> {
        let library = Arc::new(Library::new(library_path).unwrap());
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
            .unwrap()
            .read();

        if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Arc::clone(&library));

        (decl.register)(&mut registrar);

        self.functions
            .lock()
            .expect("Cannot lock Mutex")
            .extend(registrar.functions);

        self.libraries
            .lock()
            .expect("Cannot lock Mutex")
            .push(library);

        Ok(())
    }

    pub fn call(
        &self,
        function: &str,
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.functions
            .lock()
            .expect("Cannot lock Mutex")
            .get(function)
            .ok_or_else(|| format!("\"{}\" not found", function))
            .unwrap()
            .call(req)
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
