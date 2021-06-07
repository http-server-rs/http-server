use anyhow::{Error, Result};
use hyper::{Body, Request, Response};
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;

use crate::addon::logger::print::{self, Print};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    DateTime,
    HttpResponseStatus,
    HttpResponseDelay,
    HttpRequestIP,
    HttpRequestMethod,
    HttpRequestURI,
}

impl Token {
    pub fn printer(&self) -> Box<dyn Print> {
        match self {
            Token::DateTime => Box::new(print::datetime::DateTime),
            Token::HttpResponseStatus => Box::new(print::res_status::HttpResponseStatus),
            Token::HttpResponseDelay => Box::new(print::res_delay::HttpResponseDelay),
            // Token::HttpRequestIP => print::datetime::DateTime,
            // Token::HttpRequestMethod => print::datetime::DateTime,
            // Token::HttpRequestURI => print::datetime::DateTime,
            _ => {
                println!("Todo!");
                Box::new(print::datetime::DateTime)
            }
        }
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "$datetime" => Ok(Token::DateTime),
            "$res_status" => Ok(Token::HttpResponseStatus),
            "$res_delay" => Ok(Token::HttpResponseDelay),
            "$req_ip" => Ok(Token::HttpRequestIP),
            "$req_method" => Ok(Token::HttpRequestMethod),
            "$req_uri" => Ok(Token::HttpRequestURI),
            _ => Err(Error::msg(format!("Invalid token provided {}", s))),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let string = match self {
            &Token::DateTime => "$datetime",
            &Token::HttpResponseStatus => "$res_status",
            &Token::HttpResponseDelay => "$res_delay",
            &Token::HttpRequestIP => "$req_ip",
            &Token::HttpRequestMethod => "$req_method",
            &Token::HttpRequestURI => "$req_uri",
        };

        String::from(string)
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    tokens: Vec<Token>,
    logging_format: String,
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let logging_format = s.to_string();
        let tokens = s
            .split(" ")
            .map(|part| Token::from_str(part))
            .collect::<Result<Vec<Token>>>()?;

        Ok(Pattern {
            tokens,
            logging_format,
        })
    }
}

impl Pattern {
    pub fn output_string(
        &self,
        request: Arc<Request<Body>>,
        response: &mut Response<Body>,
    ) -> String {
        let mut logging_format = self.logging_format.clone();

        for token in self.tokens.iter() {
            let printer = token.printer();
            let printer_output = printer.print(Arc::clone(&request), response);

            logging_format = self.digest(logging_format, token, printer_output);
        }

        logging_format
    }

    fn digest(&self, logging_format: String, token: &Token, printer_output: String) -> String {
        let token_string = token.to_string();
        let next = logging_format.replace(&token_string, &printer_output);

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_STRING_PATTERN: &str =
        "$datetime $res_status $res_delay $req_ip $req_method $req_uri";

    #[test]
    fn extract_tokens_from_str_pattern() {
        let pattern = Pattern::from_str(DEFAULT_STRING_PATTERN).unwrap();
        let expected_tokens = vec![
            Token::DateTime,
            Token::HttpResponseStatus,
            Token::HttpResponseDelay,
            Token::HttpRequestIP,
            Token::HttpRequestMethod,
            Token::HttpRequestURI,
        ];

        assert_eq!(pattern.tokens, expected_tokens);
    }

    #[test]
    fn finds_wrong_tokens_in_str_pattern() {
        let pattern = Pattern::from_str("$datetime $foo $res_delay $req_ip $req_method $req_uri");

        assert!(pattern.is_err());
        assert_eq!(
            pattern.err().unwrap().to_string(),
            String::from("Invalid token provided $foo")
        );
    }
}
