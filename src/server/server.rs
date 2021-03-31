use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};

use crate::config::Config;

use super::handler::Handler;

pub struct Server {
    config: Config,
}

impl Server {
    pub async fn serve(&self) {
        let address = self.config.address();
        let handler = Handler::from(self.config.clone());

        let main_svc = make_service_fn(move |_| {
            // Move a clone of `handler` into the `service_fn`.
            let handler = handler.clone();

            async {
                Ok::<_, Error>(service_fn(move |req| {
                    // Clone again to ensure that `handler` outlives this closure.
                    handler.to_owned().handle_request(req)
                }))
            }
        });

        let server = hyper::Server::bind(&address).serve(main_svc);

        if self.config.verbose() {
            println!("Server binded to: {}", address.to_string());
            println!(
                "Serving directory: {}",
                self.config.root_dir().to_str().unwrap()
            );
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
