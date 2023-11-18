use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use super::{decode_uri, encode_uri};

    #[test]
    fn encodes_uri() {
        let file_path = "/these are important files/do_not_delete/file name.txt";
        let file_path = PathBuf::from_str(file_path).unwrap();
        let file_path = encode_uri(&file_path);

        assert_eq!(
            file_path,
            "/these%20are%20important%20files/do_not_delete/file%20name.txt"
        );
    }

    #[test]
    fn decodes_uri() {
        let file_path = "these%20are%20important%20files/do_not_delete/file%20name.txt";
        let file_path = decode_uri(file_path);

        assert_eq!(
            file_path,
            PathBuf::from("these are important files/do_not_delete/file name.txt")
        );
    }
}
