use serde::Deserialize;

use crate::addon::logger::Logger;

/// Configuration for HTTP server logging
pub struct LoggingConfig {
    logger: Logger,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LoggingConfigFile {
    pub template: String,
}

impl LoggingConfig {}
