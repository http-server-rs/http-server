use http_server_lib::make_server;
use std::process::exit;

#[cfg(feature = "dhat-profiling")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat-profiling")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-profiling")]
    let _dhat = Dhat::start_heap_profiling();

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
