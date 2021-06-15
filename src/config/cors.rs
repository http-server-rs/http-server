use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CorsConfig {
    pub allow_credentials: bool,
    pub allow_headers: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_origin: Option<String>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<u64>,
    pub request_headers: Option<Vec<String>>,
    pub request_method: Option<String>,
}

impl CorsConfig {
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
            allow_credentials: false,
            max_age: Some(43200),
            expose_headers: None,
            request_headers: None,
            request_method: None,
        }
    }
}
