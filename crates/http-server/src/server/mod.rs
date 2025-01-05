pub mod config;
pub mod plugin;

use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use local_ip_address::local_ip;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use self::config::Config;
use self::plugin::PluginStore;

const ALL_INTERFACES_IPV4: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub async fn run(self, rt: Arc<Runtime>) -> Result<()> {
        let addr = SocketAddr::from((self.config.host, self.config.port));
        let listener = TcpListener::bind(addr).await?;
        let plugin_store = Arc::new(PluginStore::new());
        let config = PathBuf::from_str("./config.toml")?;
        let handle = Arc::new(rt.handle().to_owned());

        println!("Listening on http://{}", addr);

        if matches!(addr.ip(), IpAddr::V4(ALL_INTERFACES_IPV4)) {
            if let Ok(local_ip) = local_ip() {
                println!("Local Network on http://{}:{}", local_ip, self.config.port);
            }
        }

        unsafe {
            plugin_store
                .load(Arc::clone(&handle), config, "file_explorer.plugin.httprs")
                .await?;
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let plugin_store = Arc::clone(&plugin_store);
            let cors = if self.config.cors {
                Some(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                )
            } else {
                None
            };

            handle.spawn(async move {
                let plugin_store = Arc::clone(&plugin_store);
                let svc = tower::service_fn(|req: Request<Incoming>| async {
                    let (parts, body) = req.into_parts();
                    let body = body.collect().await.unwrap().to_bytes();
                    println!("Exec: hello-world");
                    match plugin_store.run("hello-world", parts, body).await {
                        Ok(res) => Ok::<
                            Response<http_body_util::Full<hyper::body::Bytes>>,
                            Infallible,
                        >(res),
                        Err(err) => {
                            eprintln!("Error: {:?}", err);
                            Ok(Response::new(Full::new(Bytes::from(
                                "Internal Server Error",
                            ))))
                        }
                    }
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
