pub mod command;

use clap::Parser;

use self::command::Command;

#[derive(Debug, Parser)]
#[command(
    name = "http-server",
    author = "Esteban Borai <estebanborai@gmail.com>",
    about = "Simple and configurable command-line HTTP server\nSource: https://github.com/http-server-rs/http-server"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
