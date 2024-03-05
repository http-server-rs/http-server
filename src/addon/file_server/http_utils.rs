use chrono::{DateTime, Local, Utc};
use color_eyre::eyre::Context;
use futures::Stream;
use http::response::Builder as HttpResponseBuilder;
use hyper::body::Body;
use hyper::body::Bytes;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{self, Poll};
use tokio::io::{AsyncRead, ReadBuf};

use super::file::File;

const FILE_BUFFER_SIZE: usize = 8 * 1024;

pub type FileBuffer = Box<[MaybeUninit<u8>; FILE_BUFFER_SIZE]>;

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

impl ToString for CacheControlDirective {
    fn to_string(&self) -> String {
        match &self {
            Self::MustRevalidate => String::from("must-revalidate"),
            Self::NoCache => String::from("no-cache"),
            Self::NoStore => String::from("no-store"),
            Self::NoTransform => String::from("no-transform"),
            Self::Public => String::from("public"),
            Self::Private => String::from("private"),
            Self::ProxyRavalidate => String::from("proxy-revalidate"),
            Self::MaxAge(age) => format!("max-age={}", age),
            Self::SMaxAge(age) => format!("s-maxage={}", age),
        }
    }
}

pub struct ResponseHeaders {
    cache_control: String,
    content_length: u64,
    content_type: String,
    etag: String,
    last_modified: String,
}

impl ResponseHeaders {
    pub fn new(
        file: &File,
        cache_control_directive: CacheControlDirective,
    ) -> color_eyre::Result<ResponseHeaders> {
        let last_modified = file
            .last_modified()
            .context("Unable to get file's last modified date")?;

        Ok(ResponseHeaders {
            cache_control: cache_control_directive.to_string(),
            content_length: ResponseHeaders::content_length(file),
            content_type: ResponseHeaders::content_type(file),
            etag: ResponseHeaders::etag(file, &last_modified),
            last_modified: ResponseHeaders::last_modified(&last_modified),
        })
    }

    fn content_length(file: &File) -> u64 {
        file.size()
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
    file: File,
    cache_control_directive: CacheControlDirective,
) -> color_eyre::Result<hyper::http::Response<Body>> {
    let headers = ResponseHeaders::new(&file, cache_control_directive)
        .context("Failed to construct response headers")?;
    let builder = HttpResponseBuilder::new()
        .header(http::header::CONTENT_LENGTH, headers.content_length)
        .header(http::header::CACHE_CONTROL, headers.cache_control)
        .header(http::header::CONTENT_TYPE, headers.content_type)
        .header(http::header::ETAG, headers.etag)
        .header(http::header::LAST_MODIFIED, headers.last_modified);

    let body = file_bytes_into_http_body(file).await;
    let response = builder
        .body(body)
        .context("Failed to build HTTP File Response")?;

    Ok(response)
}

pub async fn file_bytes_into_http_body(file: File) -> Body {
    let byte_stream = ByteStream {
        file: file.file,
        buffer: Box::new([MaybeUninit::uninit(); FILE_BUFFER_SIZE]),
    };

    Body::wrap_stream(byte_stream)
}

pub struct ByteStream {
    file: tokio::fs::File,
    buffer: FileBuffer,
}

impl Stream for ByteStream {
    type Item = color_eyre::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        let ByteStream {
            ref mut file,
            ref mut buffer,
        } = *self;
        let mut read_buffer = ReadBuf::uninit(&mut buffer[..]);

        match Pin::new(file).poll_read(cx, &mut read_buffer) {
            Poll::Ready(Ok(())) => {
                let filled = read_buffer.filled();

                if filled.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Bytes::copy_from_slice(filled))))
                }
            }
            Poll::Ready(Err(error)) => Poll::Ready(Some(Err(error.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}
