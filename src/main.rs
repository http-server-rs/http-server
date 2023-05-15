use std::net::{IpAddr, SocketAddr};

use axum::{routing::get, Router};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The address to listen on
    #[arg(long, default_value = "127.0.0.1")]
    host: IpAddr,
    /// The port to listen on
    #[arg(long, default_value = "7878")]
    port: u16,
}

impl Cli {
    fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let address = cli.address();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    println!("Listening on http://{}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
