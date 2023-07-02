use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ProxyConfig {
    pub url: String,
}

impl ProxyConfig {
    pub fn new(url: String) -> Self {
        ProxyConfig { url }
    }

    pub fn url(url: String) -> Self {
        ProxyConfig { url }
    }
}
