use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_decode, utf8_percent_encode};
use std::path::{Path, PathBuf};

pub const PERCENT_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

pub fn encode_uri(file_path: &Path) -> String {
    assert!(!file_path.is_absolute());

    file_path
        .iter()
        .flat_map(|component| {
            let component = component.to_str().unwrap();
            let segment = utf8_percent_encode(component, PERCENT_ENCODE_SET);

            std::iter::once("/").chain(segment)
        })
        .collect::<String>()
}

pub fn decode_uri(file_path: &str) -> PathBuf {
    file_path
        .split('/')
        .map(|encoded_part| {
            let decode = percent_decode(encoded_part.as_bytes());
            let decode = decode.decode_utf8_lossy();

            decode.to_string()
        })
        .collect::<PathBuf>()
}
