pub mod cli;

use anyhow::Result;
use clap::Parser;

use self::cli::command::Command;
use self::cli::Cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let args = Cli::parse();

    match args.command {
        Command::Start(opt) => opt.exec()?,
        Command::Setup(opt) => opt.exec()?,
    }

    Ok(())
}
