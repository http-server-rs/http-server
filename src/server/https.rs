use anyhow::Result;
use async_stream::stream;
use core::task::{Context, Poll};
use futures::{Stream, TryFutureExt};
use hyper::server::accept::Accept;
use hyper::server::Builder;
use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use std::io::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

pub struct HttpsAcceptor<'a> {
    acceptor: Pin<Box<dyn Stream<Item = Result<TlsStream<TcpStream>, Error>> + 'a>>,
}

impl<'a> HttpsAcceptor<'a> {
    pub fn new(
        acceptor: Pin<Box<dyn Stream<Item = Result<TlsStream<TcpStream>, Error>> + 'a>>,
    ) -> Self {
        HttpsAcceptor { acceptor }
    }
}

impl Accept for HttpsAcceptor<'_> {
    type Conn = TlsStream<TcpStream>;
    type Error = Error;

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        Pin::new(&mut self.acceptor).poll_next(cx)
    }
}

pub struct Https {
    cert: Vec<Certificate>,
    key: PrivateKey,
}

impl Https {
    pub fn new(cert: Vec<Certificate>, key: PrivateKey) -> Self {
        Https { cert, key }
    }

    fn make_tls_cfg(&self) -> Result<Arc<ServerConfig>> {
        let (cert, key) = (self.cert.clone(), self.key.clone());
        let mut cfg = ServerConfig::new(NoClientAuth::new());

        cfg.set_single_cert(cert, key)?;
        cfg.set_protocols(&[b"h2".to_vec(), b"http:/1.1".to_vec()]);

        Ok(Arc::new(cfg))
    }

    pub async fn make_server(&self, addr: SocketAddr) -> Result<Builder<HttpsAcceptor<'_>>> {
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

        let acceptor = HttpsAcceptor::new(Box::pin(incoming_tls_stream));
        let server = hyper::server::Server::builder(acceptor);

        Ok(server)
    }
}
