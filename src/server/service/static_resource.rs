use http::header;
use http::response::Builder as ResponseBuilder;
use http::StatusCode;
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;
use std::convert::Infallible;

pub async fn static_resource<B>(
    req: Request<B>,
    staticfile: Static,
) -> Result<Response<Body>, Infallible> {
    if req.uri().path() == "/" {
        let res = ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, "/hyper_staticfile/")
            .body(Body::empty())
            .expect("Unable to build response");

        return Ok(res);
    }

    staticfile.clone().serve(req).await.map_err(|e| {})
}
