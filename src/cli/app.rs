use crate::cli::make_args;
use clap::{crate_version, App};

/// Creates a CLAP `App` instance
pub fn make_app() -> App<'static, 'static> {
    App::new("http-server")
        .author("Esteban Borai (https://github.com/EstebanBorai)")
        .about("A simple, zero-configuration command-line HTTP server")
        .version(crate_version!())
        .setting(clap::AppSettings::ColoredHelp)
        .args(&make_args())
}
