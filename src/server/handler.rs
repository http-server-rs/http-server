use anyhow::Result;
use http::response::Builder as ResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request, Response};

use crate::Config;

use super::service::FileExplorer;

#[derive(Clone)]
pub struct Handler {
    file_explorer: FileExplorer<'static>,
}

impl Handler {
    pub async fn handle_request(self, req: Request<Body>) -> Result<Response<Body>> {
        match (req.method(), req.uri()) {
            (&Method::GET, _) => self.file_explorer.resolve(req).await,
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
        let file_explorer = FileExplorer::new(config.root_dir());

        Handler { file_explorer }
    }
}
