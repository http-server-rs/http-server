use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::rc::Rc;

use libloading::Library;

use http_server_plugin::{
    Function, InvocationError, PluginDeclaration, CORE_VERSION, RUSTC_VERSION,
};

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct FunctionProxy {
    function: Box<dyn Function>,
    _lib: Rc<Library>,
}

impl Function for FunctionProxy {
    fn call(&self, args: &[f64]) -> Result<f64, InvocationError> {
        self.function.call(args)
    }

    fn help(&self) -> Option<&str> {
        self.function.help()
    }
}

#[derive(Default)]
pub struct ExternalFunctions {
    functions: HashMap<String, FunctionProxy>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalFunctions {
    pub fn new() -> ExternalFunctions {
        ExternalFunctions::default()
    }

    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        // load the library into memory
        let library = Rc::new(Library::new(library_path).unwrap());

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
            .unwrap()
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.functions.extend(registrar.functions);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);

        Ok(())
    }

    pub fn call(&self, function: &str, arguments: &[f64]) -> Result<f64, InvocationError> {
        self.functions
            .get(function)
            .ok_or_else(|| format!("\"{}\" not found", function))
            .unwrap()
            .call(arguments)
    }
}

struct PluginRegistrar {
    functions: HashMap<String, FunctionProxy>,
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            functions: HashMap::default(),
        }
    }
}

impl http_server_plugin::PluginRegistrar for PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>) {
        let proxy = FunctionProxy {
            function,
            _lib: Rc::clone(&self.lib),
        };

        self.functions.insert(name.to_string(), proxy);
    }
}
