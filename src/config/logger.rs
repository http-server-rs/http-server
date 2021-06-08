use serde::Deserialize;
use std::sync::Arc;

use crate::addon::logger::Logger;

const DEFAULT_LOG_PATTERN: &str =
    "$datetime $res_status $res_delay $req_ip $req_method $req_uri \n";

/// Configuration for HTTP server logger
#[derive(Clone)]
pub struct LoggerConfig {
    pub logger: Arc<Logger>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let logger = Logger::new(DEFAULT_LOG_PATTERN).unwrap();
        let logger = Arc::new(logger);

        LoggerConfig { logger }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LoggerConfigFile {
    pub template: String,
}

impl LoggerConfig {}
