use clap::{crate_authors, crate_description, crate_name, crate_version, App, ArgMatches};

use super::argument::make_arguments;

/// Creates a **clap** application, bind arguments to it and then get matches
/// from the exection
pub fn build() -> App<'static, 'static> {
    App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about("Configurable and simple command-line HTTP server\nSource: https://github.com/http-server-rs/http-server")
    .args(&make_arguments())
}
