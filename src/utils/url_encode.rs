use anyhow::{Error, Result};
use percent_encoding::{
    percent_decode, percent_encode, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC,
};
use std::path::{PathBuf, Path};
use std::str::FromStr;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

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
            let segment = match component.to_str() {
                Some(component) => utf8_percent_encode(component, PERCENT_ENCODE_SET),
                None => {
                    let bytes = {
                        #[cfg(windows)]
                        {
                            let mut bytes = Vec::with_capacity(component.len() * 2);

                            for wc in component.encode_wide() {
                                let wc = wc.to_be_bytes();

                                bytes.push(wc[0]);
                                bytes.push(wc[1]);
                            }

                            bytes
                        }

                        #[cfg(not(windows))]
                        component.as_bytes()
                    };

                    percent_encode(bytes, PERCENT_ENCODE_SET)
                }
            };

            std::iter::once("/").chain(segment)
        })
        .collect::<String>()
}

pub fn decode_uri(file_path: &str) -> Result<PathBuf> {
    let path_string = file_path
        .split('/')
        .map(|encoded_part| {
            let decode = percent_decode(encoded_part.as_bytes());
            let decode = decode.decode_utf8_lossy();

            decode.to_string()
        })
        .collect::<Vec<String>>()
        .join("/");

    PathBuf::from_str(&path_string).map_err(|err| Error::msg(err.to_string()))
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
        let file_path = decode_uri(&file_path).unwrap();
        let file_path = file_path.to_str().unwrap();

        assert_eq!(
            file_path,
            "these are important files/do_not_delete/file name.txt"
        );
    }
}
