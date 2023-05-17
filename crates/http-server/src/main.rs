mod cli;
mod server;

use clap::Parser;
use color_eyre::eyre::Result;

use self::cli::Cli;
use self::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let address = cli.address();
    let server = Server::from(cli);

    println!("Listening on http://{}", address);

    axum::Server::bind(&address)
        .serve(server.router().into_make_service())
        .await?;

    Ok(())
}