use hyper::{Body, Response};

pub fn with_cors_allow_all(response: &mut Response<Body>) {
    let headers = response.headers_mut();

    headers.append(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        hyper::header::HeaderValue::from_str("*").unwrap(),
    );
    headers.append(
        hyper::header::ACCESS_CONTROL_ALLOW_METHODS,
        hyper::header::HeaderValue::from_str("GET, POST, PUT, PATCH, DELETE").unwrap(),
    );
    headers.append(
        hyper::header::ACCESS_CONTROL_ALLOW_HEADERS,
        hyper::header::HeaderValue::from_str("Content-Type, Content-Length, Origin").unwrap(),
    );
}
