#[cfg(test)]
mod tests {
    use http::{Response, StatusCode};
    use hyper::{Body, Client};

    async fn http_get(url: &str) -> Response<Body> {
        let client = Client::default();

        client.get(url.parse().unwrap()).await.unwrap()
    }

    #[tokio::test]
    async fn cors_get_request_to_root_responds_200() {
        let response = http_get("http://0.0.0.0:7878").await;
        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_METHODS)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_HEADERS)
            .is_some());
    }

    #[tokio::test]
    async fn cors_get_request_retrieve_file() {
        let response = http_get("http://0.0.0.0:7878/docs/screenshot.png").await;
        let headers = response.headers();

        println!("{:#?}", response.headers());
        assert_eq!(response.status(), StatusCode::OK);
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_METHODS)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_HEADERS)
            .is_some());
    }

    #[tokio::test]
    async fn cors_get_request_file_not_found() {
        let response = http_get("http://0.0.0.0:7878/xyz/abc.txt").await;
        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_METHODS)
            .is_some());
        assert!(headers
            .get(http::header::ACCESS_CONTROL_ALLOW_HEADERS)
            .is_some());
    }
}
