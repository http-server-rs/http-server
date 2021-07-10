#[cfg(test)]
mod tests {
    use http::{Response, StatusCode};
    use hyper::{Body, Client};

    async fn http_get(url: &str) -> Response<Body> {
        let client = Client::default();

        client.get(url.parse().unwrap()).await.unwrap()
    }

    #[tokio::test]
    async fn defacto_get_request_to_root_responds_200() {
        let response = http_get("http://0.0.0.0:7878").await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn defacto_get_request_retrieve_file() {
        let response = http_get("http://0.0.0.0:7878/docs/screenshot.png").await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn defacto_get_request_file_not_found() {
        let response = http_get("http://0.0.0.0:7878/xyz/abc.txt").await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
