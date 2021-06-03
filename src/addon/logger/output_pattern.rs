use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Token {
    DateTime(),
}

#[derive(Clone, Debug)]
pub struct OutputPattern {
    tokens: Vec<Token>,
}

impl FromStr for OutputPattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(OutputPattern {
            tokens: vec![Token::DateTime()],
        })
    }
}
