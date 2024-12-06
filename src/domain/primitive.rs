/// Represents a sequence of characters or null.
/// For non-null strings, first the length N is given as an INT16.
/// Then N bytes follow which are the UTF-8 encoding of the character sequence.
/// A null value is encoded with length of -1 and there are no following bytes.
pub struct NullableString {
    length: i16,
    content: Vec<u8>,
}

/// todo: fix this
impl From<&[u8]> for NullableString {
    fn from(bytes: &[u8]) -> Self {
        Self {
            length: bytes.len() as i16,
            content: bytes.to_vec(),
        }
    }
}

impl Default for NullableString {
    fn default() -> Self {
        Self {
            length: -1,
            content: vec![],
        }
    }
}
