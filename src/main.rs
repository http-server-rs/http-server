mod cli;
mod server;

use clap::Parser;

use self::cli::Cli;
use self::server::Server;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let address = cli.address();
    let server = Server::from(cli);

    println!("Listening on http://{}", address);

    axum::Server::bind(&address)
        .serve(server.router().into_make_service())
        .await
        .unwrap();
}
