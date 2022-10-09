use anyhow::{Error, Result};
use hyper::header::{self, HeaderName, HeaderValue};
use std::convert::TryFrom;

use crate::config::cors::CorsConfig;

/// CORS (Cross Origin Resource Sharing) configuration for the HTTP/S
/// server.
///
/// `CorsConfig` holds the configuration for the CORS headers for a
/// HTTP/S server instance. The following headers are supported:
///
/// Access-Control-Allow-Credentials header
/// Access-Control-Allow-Headers header
/// Access-Control-Allow-Methods header
/// Access-Control-Expose-Headers header
/// Access-Control-Max-Age header
/// Access-Control-Request-Headers header
/// Access-Control-Request-Method header
///
/// Refer to CORS here: https://www.w3.org/wiki/CORS
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cors {
    /// The Access-Control-Allow-Credentials response header tells browsers
    /// whether to expose the response to frontend JavaScript code when the
    /// request's credentials mode (Request.credentials) is include.
    ///
    /// The only valid value for this header is true (case-sensitive). If you
    /// don't need credentials, omit this header entirely (rather than setting
    /// its value to false).
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Credentials
    pub(crate) allow_credentials: bool,
    /// The Access-Control-Allow-Headers response header is used in response to a
    /// preflight request which includes the Access-Control-Request-Headers to
    /// indicate which HTTP headers can be used during the actual request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers
    pub(crate) allow_headers: Option<Vec<String>>,
    /// The Access-Control-Allow-Methods response header specifies the method or
    /// methods allowed when accessing the resource in response to a preflight
    /// request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods
    pub(crate) allow_methods: Option<Vec<String>>,
    /// The Access-Control-Allow-Origin response header indicates whether the
    /// response can be shared with requesting code from the given origin.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
    pub(crate) allow_origin: Option<String>,
    /// The Access-Control-Expose-Headers response header allows a server to
    /// indicate which response headers should be made available to scripts
    /// running in the browser, in response to a cross-origin request.
    ///
    /// Only the CORS-safelisted response headers are exposed by default.
    /// For clients to be able to access other headers, the server must list them
    /// using the Access-Control-Expose-Headers header.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers
    pub(crate) expose_headers: Option<Vec<String>>,
    /// The Access-Control-Max-Age response header indicates how long the results
    /// of a preflight request (that is the information contained in the
    /// Access-Control-Allow-Methods and Access-Control-Allow-Headers headers)
    /// can be cached.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Max-Age
    pub(crate) max_age: Option<u64>,
    /// The Access-Control-Request-Headers request header is used by browsers
    /// when issuing a preflight request, to let the server know which HTTP
    /// headers the client might send when the actual request is made (such as
    /// with setRequestHeader()). This browser side header will be answered by
    /// the complementary server side header of Access-Control-Allow-Headers.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Request-Headers
    pub(crate) request_headers: Option<Vec<String>>,
    /// The Access-Control-Request-Method request header is used by browsers when
    /// issuing a preflight request, to let the server know which HTTP method will
    /// be used when the actual request is made. This header is necessary as the
    /// preflight request is always an OPTIONS and doesn't use the same method as
    /// the actual request.
    pub(crate) request_method: Option<String>,
    /// The HTTP Cross-Origin-Embedder-Policy (COEP) response header prevents a
    /// document from loading any cross-origin resources that don't explicitly
    /// grant the document permission (using CORP or CORS).
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy
    pub(crate) embedder_policy: Option<String>,
    /// The HTTP Cross-Origin-Opener-Policy (COOP) response header allows you to
    /// ensure a top-level document does not share a browsing context group with
    /// cross-origin documents.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy
    pub(crate) opener_policy: Option<String>,
}

impl Cors {
    pub fn builder() -> CorsBuilder {
        CorsBuilder {
            config: Cors::default(),
        }
    }

    pub fn make_http_headers(&self) -> Vec<(HeaderName, HeaderValue)> {
        let cors = self.clone();
        let mut cors_headers: Vec<(HeaderName, HeaderValue)> = Vec::new();

        if self.allow_credentials {
            cors_headers.push((
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_str("true").unwrap(),
            ));
        }

        if let Some(allow_headers) = cors.allow_headers {
            let allow_headers = allow_headers.join(", ");

            cors_headers.push((
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_str(allow_headers.as_str()).unwrap(),
            ));
        }

        if let Some(allow_methods) = cors.allow_methods {
            let allow_methods = allow_methods.join(", ");

            cors_headers.push((
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_str(allow_methods.as_str()).unwrap(),
            ));
        }

        if let Some(allow_origin) = cors.allow_origin {
            cors_headers.push((
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_str(allow_origin.as_str()).unwrap(),
            ));
        }

        if let Some(expose_headers) = cors.expose_headers {
            let expose_headers = expose_headers.join(", ");

            cors_headers.push((
                header::ACCESS_CONTROL_EXPOSE_HEADERS,
                HeaderValue::from_str(expose_headers.as_str()).unwrap(),
            ));
        }

        if let Some(max_age) = cors.max_age {
            cors_headers.push((
                header::ACCESS_CONTROL_MAX_AGE,
                HeaderValue::from_str(max_age.to_string().as_str()).unwrap(),
            ));
        }

        if let Some(request_headers) = cors.request_headers {
            let request_headers = request_headers.join(", ");

            cors_headers.push((
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                HeaderValue::from_str(request_headers.as_str()).unwrap(),
            ));
        }

        if let Some(request_method) = cors.request_method {
            cors_headers.push((
                header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderValue::from_str(request_method.as_str()).unwrap(),
            ));
        }

        if let Some(embedder_policy) = cors.embedder_policy {
            // TODO: Validate possible directives
            //
            // Cross-Origin-Embedder-Policy: unsafe-none | require-corp
            //
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy#directives
            cors_headers.push((
                HeaderName::from_static("Cross-Origin-Embedder-Policy"),
                HeaderValue::from_str(embedder_policy.as_str()).unwrap(),
            ));
        }

        if let Some(opener_policy) = cors.opener_policy {
            // TODO: Validate possible directives
            //
            // Cross-Origin-Opener-Policy: unsafe-none
            // Cross-Origin-Opener-Policy: same-origin-allow-popups
            // Cross-Origin-Opener-Policy: same-origin
            //
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy#directives
            cors_headers.push((
                HeaderName::from_static("Cross-Origin-Opener-Policy"),
                HeaderValue::from_str(opener_policy.as_str()).unwrap(),
            ));
        }

        cors_headers
    }
}

/// CorsConfig Builder
pub struct CorsBuilder {
    config: Cors,
}

impl CorsBuilder {
    pub fn allow_origin(mut self, origin: String) -> Self {
        self.config.allow_origin = Some(origin);
        self
    }

    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.config.allow_methods = Some(methods);
        self
    }

    pub fn allow_headers(mut self, headers: Vec<String>) -> Self {
        self.config.allow_headers = Some(headers);
        self
    }

    pub fn allow_credentials(mut self) -> Self {
        self.config.allow_credentials = true;
        self
    }

    pub fn max_age(mut self, duration: u64) -> Self {
        self.config.max_age = Some(duration);
        self
    }

    pub fn expose_headers(mut self, headers: Vec<String>) -> Self {
        self.config.expose_headers = Some(headers);
        self
    }

    pub fn request_headers(mut self, headers: Vec<String>) -> Self {
        self.config.request_headers = Some(headers);
        self
    }

    pub fn request_method(mut self, method: String) -> Self {
        self.config.request_method = Some(method);
        self
    }

    pub fn embedder_policy(mut self, embedder_policy: String) -> Self {
        self.config.embedder_policy = Some(embedder_policy);
        self
    }

    pub fn opener_policy(mut self, opener_policy: String) -> Self {
        self.config.opener_policy = Some(opener_policy);
        self
    }

    pub fn build(self) -> Cors {
        self.config
    }
}

impl TryFrom<CorsConfig> for Cors {
    type Error = Error;

    fn try_from(value: CorsConfig) -> Result<Self> {
        let mut builder = Cors::builder();

        if value.allow_credentials {
            builder = builder.allow_credentials();
        }

        if let Some(headers) = value.allow_headers {
            builder = builder.allow_headers(headers);
        }

        if let Some(methods) = value.allow_methods {
            builder = builder.allow_methods(methods);
        }

        if let Some(origin) = value.allow_origin {
            builder = builder.allow_origin(origin);
        }

        if let Some(max_age) = value.max_age {
            builder = builder.max_age(max_age);
        }

        if let Some(expose_headers) = value.expose_headers {
            builder = builder.expose_headers(expose_headers);
        }

        if let Some(request_headers) = value.request_headers {
            builder = builder.request_headers(request_headers);
        }

        if let Some(request_method) = value.request_method {
            builder = builder.request_method(request_method);
        }

        if let Some(embedder_policy) = value.embedder_policy {
            builder = builder.embedder_policy(embedder_policy);
        }

        if let Some(opener_policy) = value.opener_policy {
            builder = builder.opener_policy(opener_policy);
        }

        Ok(builder.build())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn creates_cors_config_with_builder() {
        let cors_config = Cors::builder()
            .allow_origin("http://example.com".to_string())
            .allow_methods(vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ])
            .allow_headers(vec![
                "Content-Type".to_string(),
                "Origin".to_string(),
                "Content-Length".to_string(),
            ])
            .build();

        assert_eq!(
            cors_config.allow_origin,
            Some(String::from("http://example.com"))
        );
        assert_eq!(
            cors_config.allow_methods,
            Some(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("DELETE"),
            ])
        );
        assert_eq!(
            cors_config.allow_headers,
            Some(vec![
                String::from("Content-Type"),
                String::from("Origin"),
                String::from("Content-Length"),
            ])
        );
        assert!(!cors_config.allow_credentials);
        assert_eq!(cors_config.max_age, None);
        assert_eq!(cors_config.expose_headers, None);
        assert_eq!(cors_config.request_headers, None);
        assert_eq!(cors_config.request_method, None);
    }

    #[test]
    fn creates_cors_config_which_allows_all_connections() {
        let cors_config = CorsConfig::allow_all();

        assert_eq!(cors_config.allow_origin, Some(String::from("*")));
        assert_eq!(
            cors_config.allow_methods,
            Some(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("PATCH"),
                String::from("DELETE"),
                String::from("HEAD"),
            ])
        );
        assert_eq!(
            cors_config.allow_headers,
            Some(vec![
                String::from("Origin"),
                String::from("Content-Length"),
                String::from("Content-Type"),
            ])
        );
        assert!(!cors_config.allow_credentials);
        assert_eq!(cors_config.max_age, Some(43200));
        assert_eq!(cors_config.expose_headers, None);
        assert_eq!(cors_config.request_headers, None);
        assert_eq!(cors_config.request_method, None);
    }

    #[test]
    fn creates_cors_config_from_file() {
        let allow_headers = vec![
            "content-type".to_string(),
            "content-length".to_string(),
            "request-id".to_string(),
        ];
        let allow_mehtods = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
        let allow_origin = String::from("github.com");
        let expose_headers = vec!["content-type".to_string(), "request-id".to_string()];
        let max_age = 5400;
        let request_headers = vec![
            "content-type".to_string(),
            "content-length".to_string(),
            "authorization".to_string(),
        ];
        let request_method = String::from("GET");
        let config = CorsConfig {
            allow_credentials: true,
            allow_headers: Some(allow_headers.clone()),
            allow_methods: Some(allow_mehtods.clone()),
            allow_origin: Some(allow_origin.clone()),
            expose_headers: Some(expose_headers.clone()),
            max_age: Some(max_age),
            request_headers: Some(request_headers.clone()),
            request_method: Some(request_method.clone()),
            ..Default::default()
        };
        let cors = Cors {
            allow_credentials: true,
            allow_headers: Some(allow_headers),
            allow_methods: Some(allow_mehtods),
            allow_origin: Some(allow_origin),
            expose_headers: Some(expose_headers),
            max_age: Some(max_age),
            request_headers: Some(request_headers),
            request_method: Some(request_method),
            ..Default::default()
        };

        assert_eq!(cors, Cors::try_from(config).unwrap());
    }
}
