use crate::cli::validator::{validate_address, validate_port};
use clap::Arg;

/// Tuple to hold command-line option `shor` and `long` names.
///
/// The element `0` represents the `short` name and
/// the element `1` represents the `long` name.
pub type CommandLineOption<'a> = (&'a str, &'a str);

/// The address to bind the HTTP server to
pub const ADDRESS: CommandLineOption = ("a", "address");
/// The port to bind the HTTP server to
pub const PORT: CommandLineOption = ("p", "port");
/// The root directory to serve files from
pub const ROOT_DIR: CommandLineOption = ("", "root_dir");
/// If `true` every output will not be logged
pub const SILENT: CommandLineOption = ("s", "silent");

trait IntoArg {
    /// Creates an argument with the `short` and `long` names
    /// defined in the `CommandLineOption` tuple
    fn into_arg(&self) -> Arg<'static, 'static>;
    /// Creates a positional argument with the provided `index` and `name`
    fn into_positional(&self, index: u64) -> Arg<'static, 'static>;
}

impl IntoArg for CommandLineOption<'static> {
    fn into_arg(&self) -> Arg<'static, 'static> {
        Arg::<'static, 'static>::with_name(self.1)
            .short(self.0)
            .long(self.1)
    }

    fn into_positional(&self, index: u64) -> Arg<'static, 'static> {
        Arg::with_name(self.1).index(index).required(false)
    }
}

/// Creates and returns a vector of `Arg` instances
/// with the CLI configurations
pub fn make_args() -> Vec<Arg<'static, 'static>> {
    vec![
        ADDRESS
            .into_arg()
            .help("Address to bind the server")
            .takes_value(true)
            .default_value("0.0.0.0")
            .validator(validate_address),
        PORT.into_arg()
            .help("Port to bind the server")
            .takes_value(true)
            .default_value("7878")
            .validator(validate_port),
        ROOT_DIR
            .into_positional(1)
            .help("Directory to serve files from"),
        // SILENT.into_arg().help("Disable outputs").takes_value(false),
    ]
}
