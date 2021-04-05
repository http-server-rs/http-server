use clap::{crate_authors, crate_name, crate_version, App};

pub mod argument;
mod validator;

/// Creates a **clap** application, bind arguments to it and then get matches
/// from the exection
pub fn build() -> App<'static, 'static> {
    App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about("Simple and configurable command-line HTTP server\nSource: https://github.com/EstebanBorai/http-server")
    .args(&argument::make_arguments())
}
