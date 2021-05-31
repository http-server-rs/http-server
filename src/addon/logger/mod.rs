mod output_pattern;

use anyhow::{Error, Result};
use std::sync::Arc;
use std::{fmt::Debug, str::FromStr};

use self::output_pattern::OutputPattern;

pub trait Log: Debug {
    fn log(&self) -> String;
}

pub struct Logger {
    output_pattern: OutputPattern,
    sources: Vec<Box<dyn Log + Send + Sync>>,
    print: Arc<dyn Fn(&Self) -> &str + Send + Sync>,
}

impl Logger {
    pub fn new(
        pattern: &str,
        sources: Vec<Box<dyn Log + Send + Sync>>,
        print: Box<dyn Fn(&Self) -> &str + Send + Sync>,
    ) -> Result<Self> {
        let output_pattern = OutputPattern::from_str(pattern)?;
        let print = Arc::new(print);

        Ok(Logger {
            output_pattern,
            sources,
            print,
        })
    }
}
