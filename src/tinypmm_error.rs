use std::error::Error;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum TinyppmError {
    UnsupportedBPP,
    InvalidHeader,
    FileReadError
}

impl TinyppmError {
    pub fn new (kind: TinyppmError) -> TinyppmError {
        kind
    }

    pub fn tinyppm_error_to_message(&self) -> &str {
        match *self {
            TinyppmError::InvalidHeader => "File is not proper binary .ppm file!",
            TinyppmError::UnsupportedBPP => "Only 24bpp .ppm images are supported",
            TinyppmError::FileReadError => "Error reading file!"
        }
    }
}

impl std::fmt::Display for TinyppmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.tinyppm_error_to_message())
    }
}

impl Error for TinyppmError {
    fn description(&self) -> &str {
        &self.tinyppm_error_to_message()
    }
}

impl From<std::io::Error> for TinyppmError {
    fn from(_io_error: std::io::Error) -> Self {
        TinyppmError::new(TinyppmError::FileReadError)
    }
}