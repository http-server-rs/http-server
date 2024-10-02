use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Eq)]
#[structopt(
    name = "http-server",
    author = "Esteban Borai <estebanborai@gmail.com>",
    about = "Simple and configurable command-line HTTP server\nSource: https://github.com/EstebanBorai/http-server"
)]
pub struct Cli {}
