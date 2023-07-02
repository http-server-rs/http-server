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
