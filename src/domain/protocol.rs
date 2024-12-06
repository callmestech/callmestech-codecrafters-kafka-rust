use bytes::{Buf, BufMut, BytesMut};

use super::{ApiKey, ApiKeyType, Error, NullableString};

#[derive(Debug)]
pub struct Request {
    /// The size of the header and the message.
    message_size: i32,
    header: RequestHeader,
}

impl From<&mut BytesMut> for Request {
    fn from(bytes_mut: &mut BytesMut) -> Self {
        let message_size = bytes_mut.get_i32();
        let request_api_key = bytes_mut.get_i16();
        let request_api_version = bytes_mut.get_i16();
        let corelation_id = bytes_mut.get_i32();
        let client_id = NullableString::default();
        let header = RequestHeader::new(
            request_api_key,
            request_api_version,
            corelation_id,
            client_id,
        );

        Request::new(message_size, header)
    }
}

impl Request {
    pub fn new(message_size: i32, header: RequestHeader) -> Self {
        Self {
            message_size,
            header,
        }
    }

    pub fn header(&self) -> &RequestHeader {
        &self.header
    }

    pub fn message_size(&self) -> i32 {
        self.message_size
    }
}

#[derive(Debug)]
pub struct Response {
    /// The size of the header and the message.
    message_size: i32,
    header: ResponseHeader,
}

impl Response {
    pub fn new(message_size: i32, header: ResponseHeader) -> Self {
        Self {
            message_size,
            header,
        }
    }

    pub fn message_size(&self) -> i32 {
        self.message_size
    }

    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }
}

impl From<&Request> for Response {
    fn from(req: &Request) -> Self {
        let error = if req.header().request_api_version() > 4 {
            Error::UnsupportedVersion
        } else {
            Error::NoError
        };
        let corelation_id = req.header().corelation_id();
        let api_key = ApiKey::new(ApiKeyType::ApiVersions, 0, 4);
        let response_header =
            ResponseHeader::new(corelation_id, error.error_code(), vec![api_key.clone()]);
        let response_header_bytes: BytesMut = response_header.into();
        // it's so weird that we have to create a new response header
        // i need to do it because the value is moved after calling into
        let response_header = ResponseHeader::new(corelation_id, error.error_code(), vec![api_key]);

        Self {
            message_size: response_header_bytes.len() as i32,
            header: response_header,
        }
    }
}

impl From<Response> for BytesMut {
    fn from(value: Response) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_i32(value.message_size());
        let response_header_bytes: BytesMut = value.header.into();
        bytes.extend_from_slice(&response_header_bytes);

        bytes
    }
}

/// Request Header v2
#[derive(Debug)]
pub struct RequestHeader {
    /// The API key of the request.
    request_api_key: i16,
    /// The version of the API for the request.
    request_api_version: i16,
    /// A unique identifier for the request.
    corelation_id: i32,
    /// The client id of the request.
    client_id: NullableString,
}

impl RequestHeader {
    pub fn new(
        request_api_key: i16,
        request_api_version: i16,
        corelation_id: i32,
        client_id: NullableString,
    ) -> Self {
        Self {
            request_api_key,
            request_api_version,
            corelation_id,
            client_id,
        }
    }

    pub fn request_api_key(&self) -> i16 {
        self.request_api_key
    }

    pub fn request_api_version(&self) -> i16 {
        self.request_api_version
    }

    pub fn corelation_id(&self) -> i32 {
        self.corelation_id
    }

    pub fn client_id(&self) -> &NullableString {
        &self.client_id
    }
}

impl From<BytesMut> for RequestHeader {
    fn from(mut bytes: BytesMut) -> Self {
        RequestHeader::new(
            bytes.get_i16(),
            bytes.get_i16(),
            bytes.get_i32(),
            NullableString::from(bytes),
        )
    }
}

#[derive(Debug)]
pub struct ResponseHeader {
    /// The correlation ID of the response.
    corelation_id: i32,
    /// The error code of the response.
    error_code: i16,
    api_keys: Vec<ApiKey>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    throttle_time_ms: i32,
    /// The tagged fields.
    tag_buffer: i8,
}

impl ResponseHeader {
    pub fn new(corelation_id: i32, error_code: i16, api_keys: Vec<ApiKey>) -> Self {
        Self {
            corelation_id,
            error_code,
            api_keys,
            throttle_time_ms: 420,
            tag_buffer: 0,
        }
    }

    pub fn api_keys(&self) -> &[ApiKey] {
        &self.api_keys
    }

    pub fn corelation_id(&self) -> i32 {
        self.corelation_id
    }

    pub fn error_code(&self) -> i16 {
        self.error_code
    }

    pub fn throttle_time_ms(&self) -> i32 {
        self.throttle_time_ms
    }

    pub fn tag_buffer(&self) -> i8 {
        self.tag_buffer
    }
}

impl From<&mut BytesMut> for ResponseHeader {
    fn from(value: &mut BytesMut) -> Self {
        let corelation_id = value.get_i32();
        let error_code = value.get_i16();
        let api_keys_len = value.get_i8();
        let mut api_keys_bytes = vec![0; api_keys_len as usize];
        value.copy_to_slice(&mut api_keys_bytes);
        let api_keys_bytes = BytesMut::from(&api_keys_bytes[..]);
        let mut api_keys = vec![];

        for chunk in api_keys_bytes.chunks(api_keys_bytes.len() / api_keys_len as usize) {
            let api_key = ApiKey::from(BytesMut::from(chunk));
            api_keys.push(api_key);
        }

        ResponseHeader {
            corelation_id,
            error_code,
            api_keys,
            throttle_time_ms: 420,
            tag_buffer: 0,
        }
    }
}

impl From<ResponseHeader> for BytesMut {
    fn from(value: ResponseHeader) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_i32(value.corelation_id());
        bytes.put_i16(value.error_code());

        let api_keys_len = value.api_keys().len() as i8;
        // the api_keys_len is the number of api keys plus 1
        bytes.put_i8(api_keys_len + 1);
        for api_key in value.api_keys {
            let api_key_bytes: BytesMut = api_key.into();
            bytes.extend_from_slice(&api_key_bytes);
        }
        bytes.put_i32(value.throttle_time_ms);
        bytes.put_i8(value.tag_buffer);

        bytes
    }
}
