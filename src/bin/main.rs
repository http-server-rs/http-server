use http_server::run;

fn main() {
    match run() {
        Ok(_) => println!("Execution terminated with success"),
        Err(e) => {
            eprintln!("An error ocurred executing the HTTP Server");
            eprintln!("{}", e.to_string())
        }
    }
}
