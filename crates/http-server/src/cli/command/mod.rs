mod start;

use clap::Parser;

use self::start::StartOpt;

#[derive(Debug, Parser)]
pub enum Command {
    /// Start the HTTP server
    Start(StartOpt),
}
