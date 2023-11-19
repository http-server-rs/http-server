use http::{Request, Response, StatusCode};
use hyper::{Body, Client};

async fn http_get(url: &str, accept_encoding: Option<&str>) -> Response<Body> {
    let mut request = Request::builder();
    request = request.uri(url);

    let headers = request.headers_mut().unwrap();

    if let Some(accept_encoding) = accept_encoding {
        headers.insert(
            http::header::ACCEPT_ENCODING,
            http::HeaderValue::from_str(accept_encoding).unwrap(),
        );
    }

    let client = Client::default();
    client
        .request(request.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn gzip_get_request_to_root_responds_200() {
    let response = http_get("http://0.0.0.0:7878", Some("gzip, brotli")).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(http::header::CONTENT_ENCODING)
        .is_some());
}

#[tokio::test]
async fn gzip_get_request_retrieve_image_file_not_present() {
    let response = http_get(
        "http://0.0.0.0:7878/docs/screenshot.png",
        Some("gzip, brotli"),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(http::header::CONTENT_ENCODING)
        .is_some());
}

#[tokio::test]
async fn gzip_get_request_file_not_found() {
    let response = http_get("http://0.0.0.0:7878/docs/xyz/foo.txt", Some("gzip, brotli")).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(response
        .headers()
        .get(http::header::CONTENT_ENCODING)
        .is_none());
}

#[tokio::test]
async fn gzip_no_compression_if_no_accept_encoding_header_is_provided() {
    let response = http_get("http://0.0.0.0:7878/docs/screenshot.png", None).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(http::header::CONTENT_ENCODING)
        .is_none());
}
