use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "http-server",
    author = "Esteban Borai <estebanborai@gmail.com>",
    about = "Simple and configurable command-line HTTP server\nSource: https://github.com/EstebanBorai/http-server",
    next_line_help = true
)]
pub struct Cli {
    pub host: IpAddr,
    pub port: u16,
}
