#[derive(Debug)]
pub struct HttpError {
  pub status_code: u16,
  pub message: String,
}

impl HttpError {
  pub fn new(status_code: u16, message: &str) -> Self {
    Self {
      status_code,
      message: message.to_string(),
    }
  }
}
