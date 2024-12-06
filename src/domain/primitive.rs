use bytes::{Buf, BytesMut};

/// Represents a sequence of characters or null.
/// For non-null strings, first the length N is given as an INT16.
/// Then N bytes follow which are the UTF-8 encoding of the character sequence.
/// A null value is encoded with length of -1 and there are no following bytes.
#[derive(Debug)]
pub struct NullableString {
    length: i16,
    content: Vec<u8>,
}

impl NullableString {
    pub fn new(length: i16, content: Vec<u8>) -> Self {
        Self { length, content }
    }

    pub fn length(&self) -> i16 {
        self.length
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }
}

impl From<BytesMut> for NullableString {
    fn from(mut bytes: BytesMut) -> Self {
        let length = bytes.get_i16();

        if length == -1 {
            NullableString::default()
        } else {
            let mut content = vec![0; length as usize];
            bytes.copy_to_slice(&mut content);
            NullableString::new(length, content)
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
