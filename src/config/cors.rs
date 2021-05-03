use serde::Deserialize;
use std::time::Duration;

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
#[derive(Clone, Debug)]
pub struct CorsConfig {
    /// The Access-Control-Allow-Credentials response header tells browsers
    /// whether to expose the response to frontend JavaScript code when the
    /// request's credentials mode (Request.credentials) is include.
    ///
    /// The only valid value for this header is true (case-sensitive). If you
    /// don't need credentials, omit this header entirely (rather than setting
    /// its value to false).
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Credentials
    allow_credentials: Option<()>,
    /// The Access-Control-Allow-Headers response header is used in response to a
    /// preflight request which includes the Access-Control-Request-Headers to
    /// indicate which HTTP headers can be used during the actual request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers
    allow_headers: Option<Vec<String>>,
    /// The Access-Control-Allow-Methods response header specifies the method or
    /// methods allowed when accessing the resource in response to a preflight
    /// request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods
    allow_methods: Option<Vec<String>>,
    /// The Access-Control-Allow-Origin response header indicates whether the
    /// response can be shared with requesting code from the given origin.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
    allow_origin: Option<String>,
    /// The Access-Control-Expose-Headers response header allows a server to
    /// indicate which response headers should be made available to scripts
    /// running in the browser, in response to a cross-origin request.
    ///
    /// Only the CORS-safelisted response headers are exposed by default.
    /// For clients to be able to access other headers, the server must list them
    /// using the Access-Control-Expose-Headers header.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers
    expose_headers: Option<Vec<String>>,
    /// The Access-Control-Max-Age response header indicates how long the results
    /// of a preflight request (that is the information contained in the
    /// Access-Control-Allow-Methods and Access-Control-Allow-Headers headers)
    /// can be cached.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Max-Age
    max_age: Option<Duration>,
    /// The Access-Control-Request-Headers request header is used by browsers
    /// when issuing a preflight request, to let the server know which HTTP
    /// headers the client might send when the actual request is made (such as
    /// with setRequestHeader()). This browser side header will be answered by
    /// the complementary server side header of Access-Control-Allow-Headers.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Request-Headers
    request_headers: Option<Vec<String>>,
    /// The Access-Control-Request-Method request header is used by browsers when
    /// issuing a preflight request, to let the server know which HTTP method will
    /// be used when the actual request is made. This header is necessary as the
    /// preflight request is always an OPTIONS and doesn't use the same method as
    /// the actual request.
    request_method: Option<String>,
}

impl CorsConfig {
    pub fn builder() -> CorsConfigBuilder {
        CorsConfigBuilder {
            config: CorsConfig::default(),
        }
    }

    pub fn allow_all() -> Self {
        CorsConfig {
            allow_origin: Some(String::from("*")),
            allow_methods: Some(vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "PATCH".to_string(),
                "DELETE".to_string(),
                "HEAD".to_string(),
            ]),
            allow_headers: Some(vec![
                "Origin".to_string(),
                "Content-Length".to_string(),
                "Content-Type".to_string(),
            ]),
            allow_credentials: None,
            max_age: Some(Duration::from_secs(43200)),
            expose_headers: None,
            request_headers: None,
            request_method: None,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        CorsConfig {
            allow_origin: None,
            allow_methods: None,
            allow_headers: None,
            allow_credentials: None,
            max_age: None,
            expose_headers: None,
            request_headers: None,
            request_method: None,
        }
    }
}

/// CorsConfig Builder
pub struct CorsConfigBuilder {
    config: CorsConfig,
}

impl CorsConfigBuilder {
    pub fn allow_origin(mut self, origin: &str) -> Self {
        self.config.allow_origin = Some(String::from(origin));
        self
    }

    pub fn allow_methods(mut self, methods: Vec<&str>) -> Self {
        let methods = methods
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        self.config.allow_methods = Some(methods);
        self
    }

    pub fn allow_headers(mut self, headers: Vec<&str>) -> Self {
        let headers = headers
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        self.config.allow_headers = Some(headers);
        self
    }

    pub fn allow_credentials(mut self) -> Self {
        self.config.allow_credentials = Some(());
        self
    }

    pub fn max_age(mut self, duration: Duration) -> Self {
        self.config.max_age = Some(duration);
        self
    }

    pub fn expose_headers(mut self, headers: Vec<&str>) -> Self {
        let headers = headers
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        self.config.expose_headers = Some(headers);
        self
    }

    pub fn request_headers(mut self, headers: Vec<&str>) -> Self {
        let headers = headers
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        self.config.request_headers = Some(headers);
        self
    }

    pub fn request_method(mut self, method: &str) -> Self {
        self.config.request_method = Some(String::from(method));
        self
    }

    pub fn build(self) -> CorsConfig {
        self.config
    }
}

/// CORS configuration definition for server configuration file.
/// This struct maps the values from the server configuration file
/// to a `CorsConfig` struct
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CorsConfigFile {
    pub allow_credentials: bool,
    pub allow_headers: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_origin: Option<String>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<f64>,
    pub request_headers: Option<Vec<String>>,
    pub request_method: Option<String>,
}

mod tests {
    use super::*;

    #[test]
    fn creates_cors_config_with_builder() {
        let cors_config = CorsConfig::builder()
            .allow_origin("http://example.com")
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_headers(vec!["Content-Type", "Origin", "Content-Length"])
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
        assert_eq!(cors_config.allow_credentials, None);
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
        assert_eq!(cors_config.allow_credentials, None);
        assert_eq!(cors_config.max_age, Some(Duration::from_secs(43200)));
        assert_eq!(cors_config.expose_headers, None);
        assert_eq!(cors_config.request_headers, None);
        assert_eq!(cors_config.request_method, None);
    }
}
