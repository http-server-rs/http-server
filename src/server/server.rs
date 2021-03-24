use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

use crate::config::Config;

use super::service;

pub struct Server {
    config: Config,
}

impl Server {
    pub async fn serve(&self) {
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(service::hello_world))
        });

        let address = self.config.address();
        let server = hyper::Server::bind(&address).serve(make_svc);

        if self.config.verbose {
            println!("Server binded to: {}", address.to_string());
        }

        if let Err(e) = server.await {
            eprint!("Server Error: {}", e);
        }
    }
}

impl From<Config> for Server {
    fn from(config: Config) -> Self {
        Server { config }
    }
}
