use axum::{routing::get, Router};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The address to listen on
    #[arg(long)]
    address: String,
    /// The port to listen on
    #[arg(long)]
    port: u16,
}

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
