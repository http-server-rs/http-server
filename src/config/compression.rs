use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CompressionConfig {
    pub gzip: bool,
}
