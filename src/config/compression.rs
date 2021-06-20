use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CompressionConfig {
    pub gzip: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig { gzip: false }
    }
}
