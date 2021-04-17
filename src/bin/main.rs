use http_server::make_server;

#[tokio::main]
async fn main() {
    let server = make_server().unwrap();

    server.run().await;
}
