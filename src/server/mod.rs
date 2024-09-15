pub mod handler;
pub mod middleware;

use std::sync::Arc;

use anyhow::Result;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, Response};
use hyper::http::StatusCode;
use hyper_util::rt::TokioIo;
use serde::Serialize;
use tokio::net::TcpListener;

use crate::config::Config;

use self::handler::Handler;

pub type HttpRequest = hyper::Request<hyper::body::Incoming>;

pub type HttpResponse = hyper::Response<Full<Bytes>>;

#[derive(Debug, Serialize)]
pub struct HttpErrorResponse {
    status_code: u16,
    message: Option<String>,
}

impl HttpErrorResponse {
    pub fn new(status_code: StatusCode) -> Self {
        HttpErrorResponse {
            status_code: status_code.as_u16(),
            message: None,
        }
    }

    pub fn with_message(self, message: &str) -> Self {
        HttpErrorResponse {
            message: Some(message.to_string()),
            ..self
        }
    }

    pub fn into_response(self) -> HttpResponse {
        let body = serde_json::ser::to_string(&self).unwrap();
        

        Response::builder()
            .status(self.status_code)
            .body(Full::new(Bytes::from(body)))
            .expect("Failed to build error response")
    }
}

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
