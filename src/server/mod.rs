mod handler;
mod middleware;

use std::sync::Arc;

use anyhow::Result;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::config::Config;

use self::handler::Handler;

pub type HttpRequest = hyper::Request<hyper::body::Incoming>;

pub type HttpResponse = hyper::Response<Full<Bytes>>;

pub struct Server;

impl Server {
    pub async fn run(config: Config) -> Result<()> {
        let listener = TcpListener::bind(config.address).await?;
        let handler = Arc::new(Handler::from(config));

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let handler = Arc::clone(&handler);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new().serve_connection(io, handler).await {
                    eprintln!("Error: {}", err);
                }
            });
        }
    }
}
