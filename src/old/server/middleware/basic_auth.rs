use http::{Request, StatusCode};
use http_auth_basic::Credentials;
use hyper::Body;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::basic_auth::BasicAuthConfig;
use crate::utils::error::make_http_error_response;

use super::MiddlewareBefore;

pub fn make_basic_auth_middleware(basic_auth_config: BasicAuthConfig) -> MiddlewareBefore {
    let credentials = Arc::new(Credentials::new(
        basic_auth_config.username.as_str(),
        basic_auth_config.password.as_str(),
    ));

    Box::new(move |request: Arc<Mutex<Request<Body>>>| {
        let credentials = Arc::clone(&credentials);

        Box::pin(async move {
            let request = request.lock().await;

            if let Some(auth_header_value) = request.headers().get(http::header::AUTHORIZATION) {
                let auth_header_value = auth_header_value.to_str().map_err(|err| {
                    make_http_error_response(StatusCode::BAD_REQUEST, err.to_string().as_str())
                })?;

                let incoming_credentials = Credentials::from_header(auth_header_value.to_string())
                    .map_err(|err| {
                        make_http_error_response(StatusCode::BAD_REQUEST, err.to_string().as_str())
                    })?;

                if incoming_credentials == *credentials {
                    return Ok(());
                }

                return Err(make_http_error_response(
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized",
                ));
            }

            Err(make_http_error_response(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
            ))
        })
    })
}
