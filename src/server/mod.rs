mod handler;
mod https;
mod service;

use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;

use crate::config::tls::TlsConfig;
use crate::config::Config;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server { config }
    }

    pub async fn run(&self) {
        let address = self.config.address();
        let handler = handler::Handler::from(self.config.clone());

        if self.config.tls().is_some() {
            let https_config = self.config.tls().unwrap();

            return self.serve_https(address, handler, https_config).await;
        }

        self.serve(address, handler).await;
    }

    pub async fn serve(&self, address: SocketAddr, handler: handler::Handler) {
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
            println!("Server binded");
        }

        if let Err(e) = server.await {
            eprint!("Server Error: {}", e);
        }
    }

    pub async fn serve_https(
        &self,
        address: SocketAddr,
        handler: handler::Handler,
        https_config: TlsConfig,
    ) {
        let (cert, key) = https_config.parts();
        let https_server_builder = https::Https::new(cert, key);
        let server = https_server_builder.make_server(address).await.unwrap();

        if self.config.verbose() {
            println!("Server binded with TLS");
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
