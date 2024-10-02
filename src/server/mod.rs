use std::{convert::Infallible, net::SocketAddr};

use anyhow::Result;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    Method, Request, Response,
};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub struct Server {}

impl Server {
    pub async fn run() -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            let cors = CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow requests from any origin
                .allow_origin(Any);

            tokio::spawn(async move {
                // N.B. should use tower service_fn here, since it's reuqired to be implemented tower Service trait before convert to hyper Service!
                let svc = tower::service_fn(hello);
                let svc = ServiceBuilder::new().layer(cors).service(svc);
                // Convert it to hyper service
                let svc = TowerToHyperService::new(svc);
                if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                    eprintln!("server error: {}", err);
                }
            });
        }
    }
}
