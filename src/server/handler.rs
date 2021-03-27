use anyhow::{Error, Result};
use http::header;
use http::response::Builder as ResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request, Response};
use hyper_staticfile::Static;
use std::path::PathBuf;

use crate::Config;

#[derive(Clone)]
pub struct Handler {
    root_dir: PathBuf,
    staticfile: Static,
}

impl Handler {
    pub async fn handle_request(self, req: Request<Body>) -> Result<Response<Body>> {
        match (req.method(), req.uri()) {
            (&Method::GET, _) => {
                if req.uri().path() == "/" {
                    let res = ResponseBuilder::new()
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .header(header::LOCATION, "/")
                        .body(Body::empty())
                        .expect("Unable to build response");

                    return Ok(res);
                }

                self.staticfile.serve(req).await.map_err(Error::from)
            }
            (_, _) => {
                let res = ResponseBuilder::new()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::empty())
                    .expect("Unable to build response");

                Ok(res)
            }
        }
    }
}

impl From<Config> for Handler {
    fn from(config: Config) -> Self {
        let staticfile = Static::new(config.root_dir());

        Handler {
            root_dir: config.root_dir(),
            staticfile,
        }
    }
}
