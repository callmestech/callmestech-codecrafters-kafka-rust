pub enum Error {
    UnsupportedVersion,
}

impl Error {
    pub fn error_code(&self) -> u16 {
        match self {
            Error::UnsupportedVersion => 35,
        }
    }

    pub fn error_code_as_bytes(&self) -> [u8; 2] {
        self.error_code().to_be_bytes()
    }
}
