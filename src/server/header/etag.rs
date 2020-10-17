use crate::server::header::HttpHeader;
use std::fs::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};

/// Exclamation point (!) 7-bit character representation
const EXCLAMATION_CHARACTER: u8 = b'\x21';

/// Hastag (#) 7-bit character representation
const HASHTAG_SYMBOL: u8 = b'\x23';

/// Closing Curly Brace (}) 7-bit character representation
const CLOSING_CURLY_BRACE: u8 = b'\x7e';

/// Padding character 7-bit representation
const PADDING_CHARACTER: u8 = b'\x80';

/// The ETag response-header field provides the current value of the entity tag
/// for the requested variant. The headers used with entity tags are described in
/// sections 14.24, 14.26 and 14.44. The entity tag MAY be used for comparison
/// with other entities from the same resource (see section 13.3.3).
///
/// # Anatomy
///
/// The value of an ETag must follow the ABNF form:
///
/// ```
/// entity-tag = [ weak ] opaque-tag
///
/// weak           = "W/"
/// opaque-tag     = quoted-string
/// ```
///
/// Reference: [US-ASCII Coded Character Set](https://www.w3.org/Protocols/rfc2616/rfc2616-sec2.html#sec2.2)
///
/// The anatomy of an ETag may vary based on its type.
/// An ETag could be either _weak_ or _strong_.
///
/// A _weak_ ETag is denoted by the prefix `W/`:
///
/// ```
/// ETag: W/"557b5a3d3eca3aa493d35e47e94c94a2"
/// ```
///
/// In the other hand a _strong_ Etag have no prefix.
///
/// # References
///
/// * [W3 RFC2616 - Sec 2](https://www.w3.org/Protocols/rfc2616/rfc2616-sec2.html#sec2.2)
/// * [W3 RFC2616 - Sec 14](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.19)
pub struct ETag {
    value: String,
}

impl ETag {
    /// Creates an **strong** entity tag from the
    /// provided metadata
    pub fn from_metadata(meta: &Metadata) -> Result<Self, ()> {
        let time_created = meta
            .created()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap();

        let time_modified = meta
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap();

        let tag = format!(
            "{:x}{:x}{:x}",
            time_created.subsec_millis(),
            time_modified.subsec_millis(),
            meta.len()
        );

        if ETag::is_valid_tag(&tag) {
            return Ok(Self { value: tag });
        }

        Err(())
    }

    /// Ensures that every character of the tag is a valid character.
    ///
    /// Every character (in its 7-bit representation) from the Unicode
    /// character U+0023 to the U+007E is valid for an ETag hash.
    /// Also the exclamation character which is represented by the Unicode
    /// code U+0021 and the padding character U+0080.
    fn is_valid_tag(slice: &str) -> bool {
        slice.bytes().all(|c| {
            c == EXCLAMATION_CHARACTER
                || (c >= HASHTAG_SYMBOL && c <= CLOSING_CURLY_BRACE) | (c >= PADDING_CHARACTER)
        })
    }
}

impl Into<HttpHeader> for ETag {
    fn into(self) -> HttpHeader {
        HttpHeader(String::from("ETag"), self.value)
    }
}
