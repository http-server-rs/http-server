use std::convert::TryFrom;

use futures::future;
use hyper::{Body, Response};

use crate::addon::cors::Cors;
use crate::config::cors::CorsConfig;

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
pub fn make_cors_middleware(cors_config: CorsConfig) -> MiddlewareAfter {
    let cors = Cors::try_from(cors_config).unwrap();
    let cors_headers = cors.make_http_headers();

    Box::new(move |_: _, response: &mut Response<Body>| {
        let headers = response.headers_mut();

        cors_headers.iter().for_each(|(header, value)| {
            headers.append(header, value.to_owned());
        });

        Box::pin(future::ok(()))
    })
}
