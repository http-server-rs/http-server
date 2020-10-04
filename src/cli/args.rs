use crate::cli::validator::{validate_address, validate_port};
use clap::Arg;

/// Tuple to hold command-line option `shor` and `long` names.
///
/// The element `0` represents the `short` name and
/// the element `1` represents the `long` name.
pub type CommandLineOption<'a> = (&'a str, &'a str);

pub const ADDRESS: CommandLineOption = ("a", "address");
pub const PORT: CommandLineOption = ("p", "port");
pub const SILENT: CommandLineOption = ("s", "silent");

trait IntoArg {
    /// Creates a `clap::Arg` from a `CommandLineOption`
    /// where the element at index `0` of the tuple will be the `short`
    /// argument name and the element at index `1` will be the `long`
    /// argument name
    fn into_arg(&self) -> Arg<'static, 'static>;
}

impl IntoArg for CommandLineOption<'static> {
    fn into_arg(&self) -> Arg<'static, 'static> {
        Arg::<'static, 'static>::with_name(self.1)
            .short(self.0)
            .long(self.1)
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
        SILENT.into_arg().help("Disable outputs").takes_value(false),
    ]
}
