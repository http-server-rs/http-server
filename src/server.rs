use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use local_ip_address::local_ip;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;

const ALL_INTERFACES_IPV4: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = SocketAddr::from((self.config.host, self.config.port));
        let listener = TcpListener::bind(addr).await?;
        let config = PathBuf::from_str("./config.toml")?;

        println!("Listening on http://{}", addr);

        if matches!(addr.ip(), IpAddr::V4(ALL_INTERFACES_IPV4)) {
            if let Ok(local_ip) = local_ip() {
                println!("Local Network on http://{}:{}", local_ip, self.config.port);
            }
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let cors = if self.config.cors {
                Some(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                )
            } else {
                None
            };

            tokio::spawn(async move {
                let svc = tower::service_fn(|req: Request<Incoming>| async {
                    let (parts, body) = req.into_parts();
                    let body = body.collect().await.unwrap().to_bytes();

                    let res = Response::new(Full::new(Bytes::from("Hello, World!")));
                    Ok::<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible>(res)
                });

                let svc = ServiceBuilder::new().option_layer(cors).service(svc);

                let svc = TowerToHyperService::new(svc);

                if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                    eprintln!("server error: {}", err);
                }
            });
        }
    }
}
