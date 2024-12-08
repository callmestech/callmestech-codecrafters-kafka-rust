use bytes::{Buf, BufMut, BytesMut};

pub enum ApiKeyType {
    ApiVersions = 18,
    DescribeTopicPartitions = 75,
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    /// The API index.
    api_key: i16,
    /// The minimum supported version, inclusive.
    min_version: i16,
    /// The maximum supported version, inclusive.
    max_version: i16,
    /// The tagged fields
    tag_buffer: i8,
}

impl ApiKey {
    pub fn new(r#type: ApiKeyType, min_version: i16, max_version: i16) -> Self {
        Self {
            api_key: r#type as i16,
            min_version,
            max_version,
            tag_buffer: 0,
        }
    }

    pub fn api_key(&self) -> i16 {
        self.api_key
    }

    pub fn min_version(&self) -> i16 {
        self.min_version
    }

    pub fn max_version(&self) -> i16 {
        self.max_version
    }

    pub fn tag_buffer(&self) -> i8 {
        self.tag_buffer
    }
}

impl From<BytesMut> for ApiKey {
    fn from(mut bytes: BytesMut) -> Self {
        ApiKey {
            api_key: bytes.get_i16(),
            min_version: bytes.get_i16(),
            max_version: bytes.get_i16(),
            tag_buffer: bytes.get_i8(),
        }
    }
}

impl From<ApiKey> for BytesMut {
    fn from(value: ApiKey) -> Self {
        let mut bytes = BytesMut::with_capacity(7);
        bytes.put_i16(value.api_key);
        bytes.put_i16(value.min_version);
        bytes.put_i16(value.max_version);
        bytes.put_i8(value.tag_buffer);
        bytes
    }
}
