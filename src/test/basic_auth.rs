use http::{HeaderValue, Request, Response, StatusCode};
use http_auth_basic::Credentials;
use hyper::{Body, Client};

async fn http_get(url: &str) -> Response<Body> {
    let client = Client::default();

    client.get(url.parse().unwrap()).await.unwrap()
}

async fn http_get_with_basic_auth(url: &str, username: &str, password: &str) -> Response<Body> {
    let credentials = Credentials::new(username, password);
    let mut request = Request::builder();
    request = request.uri(url);

    let headers = request.headers_mut().unwrap();

    headers.insert(
        http::header::AUTHORIZATION,
        HeaderValue::from_str(credentials.as_http_header().as_str()).unwrap(),
    );

    let client = Client::default();
    client
        .request(request.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn basic_auth_resolves_request_successfuly() {
    let response = http_get_with_basic_auth("http://0.0.0.0:7878", "john", "appleseed").await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn basic_auth_validates_wrong_credentials() {
    let response = http_get_with_basic_auth("http://0.0.0.0:7878", "somebody", "else").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn basic_auth_resolves_request_unauthorized_when_header_is_missing() {
    let response = http_get("http://0.0.0.0:7878").await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
