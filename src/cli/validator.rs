use std::net::Ipv4Addr;
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

pub fn is_valid_tls_key_alg(v: String) -> Result<(), String> {
    if v != "rsa" && v != "pkcs8" {
        return Err(format!(
            "Invalid value provided for TLS key algorithm. \"{}\". Valid are \"rsa\" and \"pkcs8\"",
            v
        ));
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

    #[test]
    fn validates_tls_algorithm_rsa() {
        assert!(is_valid_tls_key_alg("rsa".to_string()).is_ok());
    }

    #[test]
    fn validates_tls_algorithm_pkcs8() {
        assert!(is_valid_tls_key_alg("pkcs8".to_string()).is_ok());
    }

    #[test]
    fn invalidates_tls_algorithm() {
        assert!(is_valid_tls_key_alg("github".to_string()).is_err());
    }
}
