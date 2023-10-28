mod handler;
mod https;
mod service;

pub mod middleware;

use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
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

        if config.tls().is_some() {
            let https_config = config.tls().unwrap();
            let handler = handler.clone();
            let host = config.address().ip();
            let port = config.address().port().saturating_add(1);
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

        if !self.config.quiet() {
            println!("Serving HTTP: http://{}", address);

            if self.config.address().ip() == Ipv4Addr::from_str("0.0.0.0").unwrap() {
                if let Ok(ip) = local_ip_address::local_ip() {
                    println!("Local Network IP: http://{}:{}", ip, self.config.port());
                }
            }
        }

        if self.config.graceful_shutdown() {
            let graceful = server.with_graceful_shutdown(crate::utils::signal::shutdown_signal());

            if let Err(e) = graceful.await {
                eprint!("Server Error: {}", e);
            }

            return;
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
        let server = server.serve(make_service_fn(|_| {
            // Move a clone of `handler` into the `service_fn`.
            let handler = handler.clone();

            async {
                Ok::<_, Error>(service_fn(move |req| {
                    service::main_service(handler.to_owned(), req)
                }))
            }
        }));

        if !self.config.quiet() {
            println!("Serving HTTPS: http://{}", address);

            if self.config.address().ip() == Ipv4Addr::from_str("0.0.0.0").unwrap() {
                if let Ok(ip) = local_ip_address::local_ip() {
                    println!("Local Network IP: https://{}:{}", ip, self.config.port());
                }
            }
        }

        if self.config.graceful_shutdown() {
            let graceful = server.with_graceful_shutdown(crate::utils::signal::shutdown_signal());

            if let Err(e) = graceful.await {
                eprint!("Server Error: {}", e);
            }

            return;
        }

        if let Err(e) = server.await {
            eprint!("Server Error: {}", e);
        }
    }
}
