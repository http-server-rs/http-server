mod setup;
mod start;

use clap::Parser;

use self::setup::SetupOpt;
use self::start::StartOpt;

#[derive(Debug, Parser)]
pub enum Command {
    /// Setup the HTTP Server System Files
    Setup(SetupOpt),
    /// Start the HTTP Server
    Start(StartOpt),
}
