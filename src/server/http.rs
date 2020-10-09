use crate::config::Config;
use crate::file_explorer::FileExplorer;
use crate::handler::main_handler;
use std::net::SocketAddr;
use tiny_http::{Server, ServerConfig};

/// HTTP Server instance
pub struct HttpServer {
    pub server: tiny_http::Server,
    pub address: SocketAddr,
    pub must_log: bool,
    pub file_explorer: FileExplorer,
}

impl From<Config> for HttpServer {
    fn from(conf: Config) -> Self {
        let address = conf.socket_address;
        let file_explorer = FileExplorer::new(conf.root_dir);
        let server = Server::new(ServerConfig {
            addr: conf.socket_address,
            ssl: None,
        })
        .unwrap();

        Self {
            server,
            address,
            must_log: !conf.silent,
            file_explorer,
        }
    }
}

impl HttpServer {
    /// Binds the server to the specified address and listen for incomming requests
    pub fn serve(&self) {
        if self.must_log {
            println!(
                "Listening and serving HTTP on http://{}",
                self.address.to_string()
            );
        }

        for request in self.server.incoming_requests() {
            let (req, res) = main_handler(request, &self.file_explorer);
            req.respond(res).unwrap();
        }
    }
}
