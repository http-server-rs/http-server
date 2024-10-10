use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function(
        "file-explorer",
        Box::new(FileExplorer {
            path: String::from("/"),
        }),
    );
}

pub struct FileExplorer {
    pub path: String,
}

impl Function for FileExplorer {
    fn call(&self, _args: &[f64]) -> Result<f64, InvocationError> {
        Ok(0.0)
    }
}
