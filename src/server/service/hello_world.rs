use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
