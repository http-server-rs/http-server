mod core;
mod listing;
mod proto;
mod ui;
mod utils;

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use http::header::{CONTENT_DISPOSITION, CONTENT_TYPE, LOCATION};
use http::{HeaderValue, Method, Response, StatusCode};
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

            match self.file_explorer.peek(path.clone()).await {
                Ok(Entry::File(file)) => return self.serve_file(file, download).await,
                Ok(Entry::Directory(_)) => {
                    // Directory with `index.html`: serve it so browsers render the site.
                    if let Ok(Entry::File(index)) =
                        self.file_explorer.peek(path.join("index.html")).await
                    {
                        if !req.uri().path().ends_with('/') {
                            // Redirect so relative asset URLs resolve inside the directory.
                            let mut location = format!("{}/", req.uri().path());
                            if let Some(q) = req.uri().query() {
                                location.push('?');
                                location.push_str(q);
                            }
                            let mut response = Response::new(Full::new(Bytes::new()));
                            *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
                            response
                                .headers_mut()
                                .insert(LOCATION, HeaderValue::from_str(&location)?);
                            return Ok(response);
                        }
                        return self.serve_file(index, false).await;
                    }
                }
                Err(_) => {}
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
