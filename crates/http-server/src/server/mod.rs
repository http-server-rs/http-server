use std::{convert::Infallible, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::Result;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, Method, Response};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::plugin::ExternalFunctions;

pub struct Server {}

impl Server {
    pub async fn run() -> Result<()> {
        info!("Initializing server");

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await?;
        let functions = Arc::new(ExternalFunctions::new());
        let plugin_library = PathBuf::from_str("./target/debug/libfile_explorer.dylib").unwrap();
        let config = PathBuf::from_str("./config.toml").unwrap();

        unsafe {
            functions
                .load(config, plugin_library)
                .await
                .expect("Function loading failed");
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let functions = Arc::clone(&functions);

            let cors = CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow requests from any origin
                .allow_origin(Any);

            tokio::spawn(async move {
                let functions = Arc::clone(&functions);

                // N.B. should use tower service_fn here, since it's reuqired to be implemented tower Service trait before convert to hyper Service!
                let svc = tower::service_fn(|req| async {
                    match functions.call("file-explorer", req).await {
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
