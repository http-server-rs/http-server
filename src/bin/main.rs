use color_eyre::eyre::Context;
use http_server_lib::make_server;

#[cfg(feature = "dhat-profiling")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat-profiling")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    #[cfg(feature = "dhat-profiling")]
    let _dhat = Dhat::start_heap_profiling();

    color_eyre::install()?;

    make_server()
        .context("Failed to create server")?
        .run()
        .await
        .context("Error while running server")?;

    Ok(())
}
