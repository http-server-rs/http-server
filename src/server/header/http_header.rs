use ascii::AsciiString;
use tiny_http::Header;

pub struct HttpHeader(pub String, pub String);

impl From<Header> for HttpHeader {
  fn from(header: Header) -> Self {
    Self(header.field.to_string(), header.value.to_string())
  }
}

impl Into<Header> for HttpHeader {
  fn into(self) -> Header {
    Header {
      field: self.0.parse().unwrap(),
      value: AsciiString::from_ascii(self.1).unwrap(),
    }
  }
}
