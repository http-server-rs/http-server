mod handler;
mod https;
mod service;

pub mod middleware;

use color_eyre::eyre::{eyre, Context, Report};
use color_eyre::Section;
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

    pub async fn run(self) -> color_eyre::Result<()> {
        let config = Arc::clone(&self.config);
        let address = config.address;
        let handler = handler::HttpHandler::try_from(Arc::clone(&config))
            .context("Failed to create HTTP handler")?;
        let server = Arc::new(self);
        let mut server_instances: Vec<tokio::task::JoinHandle<color_eyre::Result<()>>> = Vec::new();

        if config.spa {
            let mut index_html = config.root_dir.clone();
            index_html.push("index.html");

            if !index_html.exists() {
                return Err(
                    eyre!("SPA flag is enabled, but index.html in root does not exist")
                        .with_suggestion(|| {
                            format!("Create index.html in root ({:?})", config.root_dir)
                        }),
                );
            }
        }

        if let Some(tls_config) = config.tls.clone() {
            let handler = handler.clone();
            let host = config.address.ip();
            let port = config.address.port().saturating_add(1);
            let address = SocketAddr::new(host, port);
            let server = Arc::clone(&server);
            let task = tokio::spawn(async move {
                let server = Arc::clone(&server);

                server
                    .serve_https(address, handler, tls_config)
                    .await
                    .context("Failed to serve HTTPS")?;

                Ok(())
            });

            server_instances.push(task);
        }

        let server = Arc::clone(&server);
        let task = tokio::spawn(async move {
            let server = Arc::clone(&server);

            server.serve(address, handler).await;

            Ok(())
        });

        server_instances.push(task);

        for server_task in server_instances {
            server_task.await?.context("Task failed")?;
        }

        Ok(())
    }

    pub async fn serve(&self, address: SocketAddr, handler: handler::HttpHandler) {
        let server = hyper::Server::bind(&address).serve(make_service_fn(|_| {
            // Move a clone of `handler` into the `service_fn`.
            let handler = handler.clone();

            async {
                Ok::<_, Report>(service_fn(move |req| {
                    service::main_service(handler.to_owned(), req)
                }))
            }
        }));

        if !self.config.quiet {
            println!("Serving HTTP: http://{}", address);

            if self.config.address.ip() == Ipv4Addr::from_str("0.0.0.0").unwrap() {
                if let Ok(ip) = local_ip_address::local_ip() {
                    println!("Local Network IP: http://{}:{}", ip, self.config.port);
                }
            }
        }

        if self.config.graceful_shutdown {
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
    ) -> color_eyre::Result<()> {
        let (cert, key) = https_config.parts();
        let https_server_builder = https::Https::new(cert, key);
        let server = https_server_builder
            .make_server(address)
            .await
            .context("Could not build an HTTPS server")?;
        let server = server.serve(make_service_fn(|_| {
            // Move a clone of `handler` into the `service_fn`.
            let handler = handler.clone();

            async {
                Ok::<_, Report>(service_fn(move |req| {
                    service::main_service(handler.to_owned(), req)
                }))
            }
        }));

        if !self.config.quiet {
            println!("Serving HTTPS: http://{}", address);

            if self.config.address.ip() == Ipv4Addr::from_str("0.0.0.0").unwrap() {
                if let Ok(ip) = local_ip_address::local_ip() {
                    println!("Local Network IP: https://{}:{}", ip, self.config.port);
                }
            }
        }

        if self.config.graceful_shutdown {
            let graceful = server.with_graceful_shutdown(crate::utils::signal::shutdown_signal());

            graceful.await.context("Server error")?;

            return Ok(());
        }

        server.await.context("Server error")?;

        Ok(())
    }
}
