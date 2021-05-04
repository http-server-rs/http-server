use hyper::header::{self, HeaderName, HeaderValue};
use hyper::{Body, Response};

use crate::config::Config;

use super::MiddlewareAfter;

/// Creates a CORS middleware with the configuration provided and returns it.
/// The configured headers will be appended to every HTTP Response before
/// sending such response back to the client (After Middleware)
///
/// CORS headers for every response are built on server initialization and
/// then are "appended" to Response headers on every response.
///
/// # Panics
///
/// Panics if a CORS config is not defined for the `Config` instance provided.
/// (`Config.cors` is `None`).
/// `make_cors_middlware` should only be called when a `CorsConfig` is defined.
///
/// Also panics if any CORS header value is not a valid UTF-8 string
pub fn make_cors_middleware(config: Config) -> MiddlewareAfter {
    let cors_config = config.cors().unwrap();
    let mut cors_headers: Vec<(HeaderName, HeaderValue)> = Vec::new();

    if cors_config.allow_credentials {
        cors_headers.push((
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_str("true").unwrap(),
        ));
    }

    if let Some(allow_headers) = cors_config.allow_headers {
        let allow_headers = allow_headers.join(", ");

        cors_headers.push((
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_str(allow_headers.as_str()).unwrap(),
        ));
    }

    if let Some(allow_methods) = cors_config.allow_methods {
        let allow_methods = allow_methods.join(", ");

        cors_headers.push((
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_str(allow_methods.as_str()).unwrap(),
        ));
    }

    if let Some(allow_origin) = cors_config.allow_origin {
        cors_headers.push((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str(allow_origin.as_str()).unwrap(),
        ));
    }

    if let Some(expose_headers) = cors_config.expose_headers {
        let expose_headers = expose_headers.join(", ");

        cors_headers.push((
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_str(expose_headers.as_str()).unwrap(),
        ));
    }

    if let Some(max_age) = cors_config.max_age {
        cors_headers.push((
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_str(max_age.to_string().as_str()).unwrap(),
        ));
    }

    if let Some(request_headers) = cors_config.request_headers {
        let request_headers = request_headers.join(", ");

        cors_headers.push((
            header::ACCESS_CONTROL_REQUEST_HEADERS,
            HeaderValue::from_str(request_headers.as_str()).unwrap(),
        ));
    }

    if let Some(request_method) = cors_config.request_method {
        cors_headers.push((
            header::ACCESS_CONTROL_REQUEST_METHOD,
            HeaderValue::from_str(request_method.as_str()).unwrap(),
        ));
    }

    Box::new(move |response: &mut Response<Body>| {
        let headers = response.headers_mut();

        cors_headers.iter().for_each(|(header, value)| {
            headers.append(header, value.to_owned());
        });
    })
}
