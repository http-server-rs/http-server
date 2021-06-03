mod handler;
mod https;
mod middleware;
mod service;

use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config::tls::TlsConfig;
use crate::config::Config;

pub struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn new(config: Config) -> Server {
        let config = Arc::new(config);

        Server { config }
    }

    pub async fn run(self) {
        let config = Arc::clone(&self.config);
        let address = config.address();
        let handler = handler::HttpHandler::from(Arc::clone(&config));
        let server = Arc::new(self);
        let mut server_instances: Vec<tokio::task::JoinHandle<()>> = Vec::new();

        if let Some(https_config) = config.tls().clone() {
            let handler = handler.clone();
            let host = config.address().ip();
            let port = config.address().port() + 1;
            let address = SocketAddr::new(host, port);
            let server = Arc::clone(&server);
            let task = tokio::spawn(async move {
                let server = Arc::clone(&server);

                server.serve_https(address, handler, https_config).await;
            });

            server_instances.push(task);
        }

        let server = Arc::clone(&server);
        let task = tokio::spawn(async move {
            let server = Arc::clone(&server);

            server.serve(address, handler).await;
        });

        server_instances.push(task);

        for server_task in server_instances {
            server_task.await.unwrap();
        }
    }

    pub async fn serve(&self, address: SocketAddr, handler: handler::HttpHandler) {
        let server = hyper::Server::bind(&address).serve(make_service_fn(|_| {
            // Move a clone of `handler` into the `service_fn`.
            let handler = handler.clone();

            async {
                Ok::<_, Error>(service_fn(move |req| {
                    service::main_service(handler.to_owned(), req)
                }))
            }
        }));

        if self.config.verbose() {
            println!("Serving HTTP: {}", address.to_string());
        }

        if let Err(e) = server.await {
            eprint!("Server Error: {}", e);
        }
    }

    pub async fn serve_https(
        &self,
        address: SocketAddr,
        handler: handler::HttpHandler,
        https_config: TlsConfig,
    ) {
        let (cert, key) = https_config.parts();
        let https_server_builder = https::Https::new(cert, key);
        let server = https_server_builder.make_server(address).await.unwrap();

        if self.config.verbose() {
            println!("Serving HTTPS: {}", address.to_string());
        }

        if let Err(e) = server
            .serve(make_service_fn(|_| {
                // Move a clone of `handler` into the `service_fn`.
                let handler = handler.clone();

                async {
                    Ok::<_, Error>(service_fn(move |req| {
                        service::main_service(handler.to_owned(), req)
                    }))
                }
            }))
            .await
        {
            eprint!("Server Error: {}", e);
        }
    }
}
