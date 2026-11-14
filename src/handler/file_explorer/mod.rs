mod core;
mod listing;
mod proto;
mod ui;
mod utils;

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use http::{HeaderValue, Method, Response};
use http_body_util::{BodyExt, Full};
use topcoat::router::{Body, Router};

use self::core::Entry;
use self::utils::decode_uri;
use crate::handler::Handler;
use crate::server::{HttpRequest, HttpResponse};

/// Server-side rendered File Explorer.
///
/// Directories are rendered by the embedded Topcoat router. Files are served
/// as-is (or as an attachment when `?download` is present).
pub struct FileExplorer {
    file_explorer: core::FileExplorer,
    router: Router,
}

impl FileExplorer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_explorer: core::FileExplorer::new(path.clone()),
            router: ui::router(path),
        }
    }

    async fn serve_file(
        &self,
        mut file: Box<core::fs::file::File>,
        download: bool,
    ) -> Result<HttpResponse> {
        let mime = file.mime().to_string();
        let name = file
            .path
            .file_name()
            .map(|n| n.to_string_lossy().replace('"', ""))
            .unwrap_or_default();
        let mut response = Response::new(Full::new(Bytes::from(file.bytes().await?)));

        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_str(&mime)?);

        if download
            && let Ok(value) = HeaderValue::from_str(&format!("attachment; filename=\"{name}\""))
        {
            response.headers_mut().insert(CONTENT_DISPOSITION, value);
        }

        Ok(response)
    }
}

#[async_trait]
impl Handler for FileExplorer {
    async fn handle(&self, req: HttpRequest) -> Result<HttpResponse> {
        if req.method() == Method::GET || req.method() == Method::HEAD {
            let path = decode_uri(req.uri().path().trim_start_matches('/'));
            let download = req
                .uri()
                .query()
                .is_some_and(|q| q.split('&').any(|p| p == "download"));

            if let Ok(Entry::File(file)) = self.file_explorer.peek(path).await {
                return self.serve_file(file, download).await;
            }
        }

        let (parts, body) = req.into_parts();
        let response = self
            .router
            .handle(http::Request::from_parts(parts, Body::new(body)))
            .await;
        let (parts, body) = response.into_parts();
        let bytes = body
            .collect()
            .await
            .map_err(|err| anyhow::anyhow!("Failed to read response body: {err}"))?
            .to_bytes();

        Ok(Response::from_parts(parts, Full::new(bytes)))
    }
}
