use std::fmt::Display;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use http::response::Builder as HttpResponseBuilder;
use http_body_util::Full;
use hyper::body::Bytes;

use crate::server::HttpResponse;

use super::file::File;

/// HTTP Response `Cache-Control` directive
///
/// Allow dead code until we have support for cache control configuration
#[allow(dead_code)]
pub enum CacheControlDirective {
    /// Cache-Control: must-revalidate
    MustRevalidate,
    /// Cache-Control: no-cache
    NoCache,
    /// Cache-Control: no-store
    NoStore,
    /// Cache-Control: no-transform
    NoTransform,
    /// Cache-Control: public
    Public,
    /// Cache-Control: private
    Private,
    /// Cache-Control: proxy-revalidate
    ProxyRavalidate,
    /// Cache-Control: max-age=<seconds>
    MaxAge(u64),
    /// Cache-Control: s-maxage=<seconds>
    SMaxAge(u64),
}

impl Display for CacheControlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::MustRevalidate => write!(f, "must-revalidate"),
            Self::NoCache => write!(f, "no-cache"),
            Self::NoStore => write!(f, "no-store"),
            Self::NoTransform => write!(f, "no-transform"),
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::ProxyRavalidate => write!(f, "proxy-revalidate"),
            Self::MaxAge(age) => write!(f, "max-age={}", age),
            Self::SMaxAge(age) => write!(f, "s-maxage={}", age),
        }
    }
}

#[derive(Debug)]
pub struct ResponseHeaders {
    cache_control: String,
    content_type: String,
    etag: String,
    last_modified: String,
}

impl ResponseHeaders {
    pub fn new(
        file: &File,
        cache_control_directive: CacheControlDirective,
    ) -> Result<ResponseHeaders> {
        let last_modified = file.last_modified()?;

        Ok(ResponseHeaders {
            cache_control: cache_control_directive.to_string(),
            content_type: ResponseHeaders::content_type(file),
            etag: ResponseHeaders::etag(file, &last_modified),
            last_modified: ResponseHeaders::last_modified(&last_modified),
        })
    }

    fn content_type(file: &File) -> String {
        file.mime().to_string()
    }

    fn etag(file: &File, last_modified: &DateTime<Local>) -> String {
        format!(
            "W/\"{0:x}-{1:x}.{2:x}\"",
            file.size(),
            last_modified.timestamp(),
            last_modified.timestamp_subsec_nanos(),
        )
    }

    fn last_modified(last_modified: &DateTime<Local>) -> String {
        format!(
            "{} GMT",
            last_modified
                .with_timezone(&Utc)
                .format("%a, %e %b %Y %H:%M:%S")
        )
    }
}

pub async fn make_http_file_response(
    mut file: File,
    cache_control_directive: CacheControlDirective,
) -> Result<HttpResponse> {
    let headers = ResponseHeaders::new(&file, cache_control_directive)?;
    let builder = HttpResponseBuilder::new()
        .header(http::header::CACHE_CONTROL, headers.cache_control)
        .header(http::header::CONTENT_TYPE, headers.content_type)
        .header(http::header::ETAG, headers.etag)
        .header(http::header::LAST_MODIFIED, headers.last_modified);

    let body = Full::new(Bytes::from(file.bytes().await?));
    let response = builder
        .body(body)
        .context("Failed to build HTTP File Response")?;

    Ok(response)
}
