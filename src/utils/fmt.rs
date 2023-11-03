/// Byte size units
const BYTE_SIZE_UNIT: [&str; 9] = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

/// Calculates the format of the `Bytes` by converting `bytes` to the
/// corresponding unit and returns a human readable size label
pub fn format_bytes(bytes: f64) -> String {
    if bytes == 0. {
        return String::from("0 Bytes");
    }

    let i = (bytes.log10() / 1024_f64.log10()).floor();
    let value = bytes / 1024_f64.powf(i);

    format!("{:.2} {}", value, BYTE_SIZE_UNIT[i as usize])
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::format_bytes;

    #[test]
    fn formats_bytes() {
        let byte_sizes = vec![1024., 1048576., 1073741824., 1099511627776.];

        let expect = vec![
            String::from("1.00 KB"),
            String::from("1.00 MB"),
            String::from("1.00 GB"),
            String::from("1.00 TB"),
        ];

        for (idx, size) in byte_sizes.into_iter().enumerate() {
            assert_eq!(format_bytes(size), expect[idx]);
        }
    }
}
