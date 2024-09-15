use std::sync::Arc;

use http_auth_basic::Credentials;
use hyper::header::AUTHORIZATION;
use hyper::http::StatusCode;

use crate::server::{middleware::MiddlewareBefore, HttpErrorResponse, HttpRequest};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String,
}

impl BasicAuthConfig {
    pub fn new(username: String, password: String) -> Self {
        BasicAuthConfig { username, password }
    }
}

pub fn make_basic_auth_middleware(basic_auth_config: &BasicAuthConfig) -> MiddlewareBefore {
    let credentials = Arc::new(Credentials::new(
        basic_auth_config.username.as_str(),
        basic_auth_config.password.as_str(),
    ));

    Box::new(move |request: HttpRequest| {
        let secret = Arc::clone(&credentials);

        Box::pin(async move {
            let auth_header = request
                .headers()
                .get(AUTHORIZATION)
                .ok_or(
                    HttpErrorResponse::new(StatusCode::UNAUTHORIZED)
                        .with_message("Missing Authorization header"),
                )
                .map_err(|err| err.into_response())?;

            let auth_header = auth_header.to_str().map_err(|err| {
                HttpErrorResponse::new(StatusCode::BAD_REQUEST)
                    .with_message("Invalid Authorization Header value")
                    .into_response()
            })?;

            let credentials = Credentials::from_header(auth_header.to_string()).map_err(|err| {
                HttpErrorResponse::new(StatusCode::UNAUTHORIZED)
                    .with_message(err.to_string().as_str())
                    .into_response()
            })?;

            if credentials == *secret {
                return Ok(request);
            }

            Err(HttpErrorResponse::new(StatusCode::UNAUTHORIZED)
                .with_message("Invalid credentials")
                .into_response())
        })
    })
}
