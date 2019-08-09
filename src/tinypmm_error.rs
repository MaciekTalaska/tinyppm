use std::error::Error;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum TinyppmError {
    Not24bpp,
    InvalidHeader,
    FileSizeMismatch,
    FileNotFound
}

impl TinyppmError {
    pub fn new (kind: TinyppmError) -> TinyppmError {
        kind
    }

    pub fn tinyppm_error_to_message(&self) -> &str {
        match *self {
            TinyppmError::FileSizeMismatch => "Invalid file size. Unable to read enough data!",
            TinyppmError::InvalidHeader => "File is not proper binary .ppm file!",
            TinyppmError::Not24bpp => "Only 24bpp .ppm images are supported",
            TinyppmError::FileNotFound => "Unable to open specified file!"
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
    fn from(io_error: std::io::Error) -> Self {
        // this could be rather sth like a general 'problem reading file' error
        TinyppmError::new(TinyppmError::FileNotFound)
    }
}