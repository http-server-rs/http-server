use crate::cli::make_args;
use clap::{crate_version, App};

/// Creates a CLAP `App` instance
pub fn make_app() -> App<'static, 'static> {
    App::new("http-server")
        .about("Command-line HTTP Server")
        .author("Authors: https://github.com/EstebanBorai/http-server/blob/main/AUTHORS")
        .version(crate_version!())
        .setting(clap::AppSettings::ColoredHelp)
        .args(&make_args())
}
