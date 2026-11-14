use anyhow::Error;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum SortBy {
    Name,
    Size,
    DateCreated,
    DateModified,
}

impl FromStr for SortBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_ascii_lowercase();
        let lower = lower.as_str();

        match lower {
            "name" => Ok(Self::Name),
            "size" => Ok(Self::Size),
            "date_created" => Ok(Self::DateCreated),
            "date_modified" => Ok(Self::DateModified),
            _ => Err(Error::msg("Value doesnt correspond")),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct QueryParams {
    pub(crate) sort_by: Option<SortBy>,
}

impl FromStr for QueryParams {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut query_params = QueryParams::default();

        for set in s.split('&') {
            let mut it = set.split('=').take(2);

            if let (Some(key), Some(value)) = (it.next(), it.next()) {
                match key {
                    "sort_by" => {
                        if let Ok(sort_value) = SortBy::from_str(value) {
                            query_params.sort_by = Some(sort_value);
                        }
                    }
                    _ => continue,
                }
            }

            continue;
        }

        Ok(query_params)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{QueryParams, SortBy};

    #[test]
    fn builds_query_params_from_str() {
        let uri_string = "sort_by=name";
        let have = QueryParams::from_str(uri_string).unwrap();
        let expect = QueryParams {
            sort_by: Some(SortBy::Name),
        };

        assert_eq!(have, expect);
    }
}
