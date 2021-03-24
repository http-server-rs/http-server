use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;

pub fn is_valid_host(v: String) -> Result<(), String> {
    if let Err(e) = Ipv4Addr::from_str(v.as_str()) {
        return Err(format!("Invalid host value provided. {}", e.to_string()));
    }

    Ok(())
}

pub fn is_valid_port(v: String) -> Result<(), String> {
    if let Err(e) = v.parse::<u16>() {
        return Err(format!("Invalid port value provided. {}", e.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_host() {
        assert!(is_valid_host("127.0.0.1".to_string()).is_ok());
    }

    #[test]
    fn invalidates_host() {
        assert!(is_valid_host("foobar".to_string()).is_err());
    }

    #[test]
    fn validates_port() {
        assert!(is_valid_port("4500".to_string()).is_ok());
    }

    #[test]
    fn invalidates_port() {
        assert!(is_valid_port("128785425".to_string()).is_err());
    }
}
