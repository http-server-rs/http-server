use http_server::make_server;
use std::process::exit;

#[tokio::main]
async fn main() {
    match make_server() {
        Ok(server) => {
            server.run().await;
        }
        Err(error) => {
            eprint!("{:?}", error);
            exit(1);
        }
    }
}
