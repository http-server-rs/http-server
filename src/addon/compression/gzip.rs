use anyhow::{Error, Result};
use flate2::write::GzEncoder;
use http::{HeaderValue, Request, Response};
use hyper::body::aggregate;
use hyper::body::Buf;
use hyper::Body;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Content-Type values that should be ignored by the compression algorithm
const IGNORED_CONTENT_TYPE: [&str; 6] = [
    "application/gzip",
    "application/octet-stream",
    "application/wasm",
    "application/zip",
    "image",
    "video",
];

pub async fn is_encoding_accepted(request: Arc<Mutex<Request<Body>>>) -> Result<bool> {
    if let Some(accept_encoding) = request
        .lock()
        .await
        .headers()
        .get(http::header::ACCEPT_ENCODING)
    {
        let accept_encoding = accept_encoding.to_str()?;

        return Ok(accept_encoding
            .split(", ")
            .map(|accepted_encoding| accepted_encoding.trim())
            .any(|accepted_encoding| accepted_encoding == "gzip"));
    }

    Ok(false)
}

pub async fn is_compressable_content_type(response: Arc<Mutex<Response<Body>>>) -> Result<bool> {
    if let Some(content_type) = response
        .lock()
        .await
        .headers()
        .get(http::header::CONTENT_TYPE)
    {
        let content_type = content_type.to_str()?;

        if IGNORED_CONTENT_TYPE.contains(&content_type) {
            return Ok(false);
        }

        return Ok(true);
    }

    Ok(false)
}

pub async fn should_compress(
    request: Arc<Mutex<Request<Body>>>,
    response: Arc<Mutex<Response<Body>>>,
) -> Result<bool> {
    Ok(is_encoding_accepted(request).await?
        && is_compressable_content_type(Arc::clone(&response)).await?)
}

pub fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
    let buffer: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut compressor: GzEncoder<Vec<u8>> = GzEncoder::new(buffer, flate2::Compression::default());

    compressor.write_all(bytes)?;

    compressor.finish().map_err(Error::from)
}

pub async fn compress_http_response(
    request: Arc<Mutex<Request<Body>>>,
    response: Arc<Mutex<Response<Body>>>,
) -> Result<()> {
    if let Ok(compressable) = should_compress(Arc::clone(&request), Arc::clone(&response)).await {
        if compressable {
            let mut buffer: Vec<u8> = Vec::new();

            {
                let mut response = response.lock().await;

                if response.headers().get("Content-Encoding").is_some() {
                    // if the "Content-Encoding" HTTP header is present in the
                    // `Response`, skip compression process
                    return Ok(());
                }

                let body = response.body_mut();
                let mut buffer_cursor = aggregate(body).await.unwrap();

                while buffer_cursor.has_remaining() {
                    buffer.push(buffer_cursor.get_u8());
                }
            }

            let compressed = compress(&buffer)?;
            let mut response = response.lock().await;
            let response_headers = response.headers_mut();

            response_headers.append(
                http::header::CONTENT_ENCODING,
                HeaderValue::from_str("gzip").unwrap(),
            );

            response_headers.remove(http::header::CONTENT_LENGTH);

            *response.body_mut() = Body::from(compressed);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use http::response::Builder as HttpResponseBuilder;
    use hyper::{Body, Request};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use crate::server::middleware;

    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn make_gzip_request_response(
        accept_encoding_gzip: bool,
    ) -> (middleware::Request<Body>, middleware::Response<Body>) {
        let file = std::include_bytes!("../../../assets/test_file.hbs");
        let request = if accept_encoding_gzip {
            let mut req = Request::new(Body::empty());

            req.headers_mut().append(
                http::header::ACCEPT_ENCODING,
                HeaderValue::from_str("gzip, deflate").unwrap(),
            );

            Arc::new(Mutex::new(req))
        } else {
            Arc::new(Mutex::new(Request::new(Body::empty())))
        };
        let response_builder =
            HttpResponseBuilder::new().header(http::header::CONTENT_TYPE, "text/html");
        let response_body = Body::from(file.to_vec());

        let response = response_builder.body(response_body).unwrap();
        let response = Arc::new(Mutex::new(response));

        (request, response)
    }

    #[test]
    fn gzip_compression_header() {
        let raw = b"aabbaabbaabbaabb\n";
        let compressed = compress(raw).unwrap();
        let expect: [u8; 27] = [
            31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 75, 76, 76, 74, 74, 68, 194, 92, 0, 169, 225, 127,
            69, 17, 0, 0, 0,
        ];

        assert_eq!(compressed, expect);
    }

    #[tokio::test]
    async fn content_encoding_gzip() {
        let (request, response) = make_gzip_request_response(true);

        compress_http_response(request, Arc::clone(&response))
            .await
            .unwrap();

        let compressed_response = response.lock().await;

        assert_eq!(
            compressed_response
                .headers()
                .get(http::header::CONTENT_ENCODING)
                .unwrap(),
            "gzip"
        );
    }

    #[tokio::test]
    async fn compresses_body() {
        let (request, response) = make_gzip_request_response(true);
        let mut body_buffer = Vec::new();
        let mut compressed_body_buffer: Vec<u8> = Vec::new();

        {
            let mut response = response.lock().await;
            let body = response.body_mut();

            let mut buffer_cursor = aggregate(body).await.unwrap();

            while buffer_cursor.has_remaining() {
                body_buffer.push(buffer_cursor.get_u8());
            }
        }

        compress_http_response(request, Arc::clone(&response))
            .await
            .unwrap();

        {
            let mut compressed_response = response.lock().await;
            let compressed_body = compressed_response.body_mut();

            let mut buffer_cursor = aggregate(compressed_body).await.unwrap();

            while buffer_cursor.has_remaining() {
                compressed_body_buffer.push(buffer_cursor.get_u8());
            }
        }

        assert_eq!(body_buffer.len(), 6364);
        assert_eq!(compressed_body_buffer.len(), 20);
    }
}
