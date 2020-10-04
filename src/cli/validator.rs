use std::net::IpAddr;
use std::str::FromStr;

/// Validate a `String` to be a valid IP Address
pub fn validate_address(value: String) -> Result<(), String> {
    match IpAddr::from_str(value.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("The address {} is not a valid IP address", value)),
    }
}

/// Validate a `String` to be a valid number and also to be a number lower than or equal to 65535
pub fn validate_port(value: String) -> Result<(), String> {
    match value.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "The provided value must be a number and must be a 16-bit integer (maximum: 65535)"
        )),
    }
}
