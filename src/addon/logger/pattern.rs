use anyhow::{Error, Result};
use hyper::{Body, Request, Response};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    DateTime,
    HttpResponseStatus,
    HttpResponseDelay,
    HttpRequestIP,
    HttpRequestMethod,
    HttpRequestURI,
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

#[derive(Clone, Debug)]
pub struct Pattern {
    tokens: Vec<Token>,
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens = s
            .split(" ")
            .map(|part| Token::from_str(part))
            .collect::<Result<Vec<Token>>>()?;

        Ok(Pattern { tokens })
    }
}

impl Pattern {
    pub fn output_string(
        &self,
        request: Arc<Request<Body>>,
        response: &mut Response<Body>,
    ) -> String {
        String::default()
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
