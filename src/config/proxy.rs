use http::Uri;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProxyConfig {
    #[serde(with = "uri_serde")]
    pub uri: Uri,
}

impl ProxyConfig {
    pub fn new(uri: Uri) -> Self {
        ProxyConfig { uri }
    }

    pub fn url(uri: Uri) -> Self {
        ProxyConfig { uri }
    }
}

mod uri_serde {
    use http::uri::InvalidUri;
    use serde::{de::Error as _, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<http::Uri, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let uri = string
            .parse()
            .map_err(|err: InvalidUri| D::Error::custom(err.to_string()))?;

        Ok(uri)
    }

    pub fn serialize<S>(uri: &http::Uri, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uri.to_string())
    }
}
