use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    Method, Request, Response,
};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;
use crate::plugin::ExternalFunctions;

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
        let functions = Arc::new(ExternalFunctions::new());
        let plugin_library = PathBuf::from_str("./target/debug/libfile_explorer.dylib").unwrap();
        let config = PathBuf::from_str("./config.toml").unwrap();
        let handle = Arc::new(rt.handle().to_owned());

        unsafe {
            functions
                .load(Arc::clone(&handle), config, plugin_library)
                .await
                .expect("Function loading failed");
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let functions = Arc::clone(&functions);
            let cors = CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any);

            handle.spawn(async move {
                let functions = Arc::clone(&functions);
                let svc = tower::service_fn(|req: Request<Incoming>| async {
                    let (parts, body) = req.into_parts();
                    let body = body.collect().await.unwrap().to_bytes();

                    match functions.call("file-explorer", parts, body).await {
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
                let svc = TowerToHyperService::new(svc);

                if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                    eprintln!("server error: {}", err);
                }
            });
        }
    }
}
