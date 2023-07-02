use http::{Response, StatusCode};
use hyper::Body;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponseBody {
    status_code: u16,
    message: String,
}

pub fn make_http_error_response(status: StatusCode, message: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(
            serde_json::ser::to_string(&ErrorResponseBody {
                status_code: status.as_u16(),
                message: message.to_string(),
            })
            .unwrap(),
        ))
        .unwrap()
}
