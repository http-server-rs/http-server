mod output_pattern;

use anyhow::{Error, Result};
use std::{fmt::Debug, str::FromStr};

use self::output_pattern::OutputPattern;

pub trait Log: Debug {
    fn log(&self) -> String;
}

pub struct Logger {
    output_pattern: OutputPattern,
    sources: Vec<Box<dyn Log>>,
    print: Box<dyn Fn(&Self) -> &str>,
}

impl Logger {
    pub fn new(
        pattern: &str,
        sources: Vec<Box<dyn Log>>,
        print: Box<dyn Fn(&Self) -> &str>,
    ) -> Result<Self> {
        let output_pattern = OutputPattern::from_str(pattern)?;

        Ok(Logger {
            output_pattern,
            sources,
            print,
        })
    }
}
