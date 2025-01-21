use anyhow::Result;
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderValue, Response};
use http_body_util::Full;
use rust_embed::Embed;

use crate::server::{HttpRequest, HttpResponse};

#[derive(Embed)]
#[folder = "../file-explorer-ui/public/dist"]
struct FileExplorerAssets;

pub struct FileExplorer;

impl FileExplorer {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        let (parts, _) = req.into_parts();
        tracing::info!("Handling request: {:?}", parts);

        // if parts.uri.path().starts_with("/api/v1") {
        //     self.handle_api(parts, body).await
        // } else {
        let path = parts.uri.path();
        let path = path.strip_prefix('/').unwrap_or(path);

        if let Some(file) = FileExplorerAssets::get(path) {
            let content_type = mime_guess::from_path(path).first_or_octet_stream();
            let content_type = HeaderValue::from_str(content_type.as_ref()).unwrap();
            let body = Full::new(Bytes::from(file.data.to_vec()));
            let mut response = Response::new(body);
            let mut headers = response.headers().clone();

            headers.append(CONTENT_TYPE, content_type);
            *response.headers_mut() = headers;

            return Ok(response);
        }

        let index = FileExplorerAssets::get("index.html").unwrap();
        let body = Full::new(Bytes::from(index.data.to_vec()));
        let mut response = Response::new(body);
        let mut headers = response.headers().clone();

        headers.append(CONTENT_TYPE, "text/html".try_into().unwrap());
        *response.headers_mut() = headers;

        Ok(response)
    }
}
