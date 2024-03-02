use http::{Request, Response};
use hyper::Body;

use super::handler::HttpHandler;

pub async fn main_service(
    handler: HttpHandler,
    req: Request<Body>,
) -> color_eyre::Result<Response<Body>> {
    handler.handle_request(req).await
}
