use std::fmt;

#[derive(Debug)]
pub struct Error {
    error: ErrorType,
}

impl Error {
    pub fn new(e: ErrorType) -> Error {
        Error { error: e }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:#?}", self.error)
    }
}

#[derive(Debug)]
pub enum ErrorType {
    InaccessibleMemoryAddress,
    InvalidOpcode,
}