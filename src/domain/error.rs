pub enum Error {
    UnsupportedVersion,
    NoError,
}

impl Error {
    pub fn error_code(&self) -> i16 {
        match self {
            Error::UnsupportedVersion => 35,
            Error::NoError => 0,
        }
    }

    pub fn error_code_as_bytes(&self) -> [u8; 2] {
        self.error_code().to_be_bytes()
    }
}
