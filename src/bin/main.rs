use http_server::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => println!("Execution terminated with success"),
        Err(e) => {
            eprintln!("An error ocurred executing the HTTP Server");
            eprintln!("{}", e.to_string())
        }
    }
}
