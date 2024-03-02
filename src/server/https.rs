use async_stream::stream;
use color_eyre::eyre::eyre;
use futures::TryFutureExt;
use hyper::server::accept::Accept;
use hyper::server::Builder;
use rustls::{Certificate, PrivateKey, ServerConfig};
use std::io::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

pub struct Https {
    cert: Vec<Certificate>,
    key: PrivateKey,
}

impl Https {
    pub fn new(cert: Vec<Certificate>, key: PrivateKey) -> Self {
        Https { cert, key }
    }

    fn make_tls_cfg(&self) -> color_eyre::Result<Arc<ServerConfig>> {
        let (certs, private_key) = (self.cert.clone(), self.key.clone());
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|err| eyre!(err))?;

        Ok(Arc::new(config))
    }

    pub async fn make_server(
        &self,
        addr: SocketAddr,
    ) -> color_eyre::Result<Builder<impl Accept<Conn = TlsStream<TcpStream>, Error = Error>>> {
        let tcp = TcpListener::bind(addr).await?;
        let tls_cfg = self.make_tls_cfg()?;
        let tls_acceptor = TlsAcceptor::from(tls_cfg);

        let incoming_tls_stream = stream! {
            loop {
                let (socket, _) = tcp.accept().await?;
                let stream = tls_acceptor.accept(socket).map_err(|error| {
                    println!("HTTPS Error: {:?}", error);

                    error
                });

                yield stream.await;
            }
        };

        let acceptor = hyper::server::accept::from_stream(incoming_tls_stream);
        let server = hyper::server::Server::builder(acceptor);

        Ok(server)
    }
}
