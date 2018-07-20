use super::super::types::Header;

use std::error::Error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum DecodingError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    Malformed,
    Forbidden,
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoding failed")
    }
}

impl Error for DecodingError {}

impl From<io::Error> for DecodingError {
    fn from(err: io::Error) -> Self {
        DecodingError::IoError(err)
    }
}

impl From<FromUtf8Error> for DecodingError {
    fn from(err: FromUtf8Error) -> Self {
        DecodingError::Utf8Error(err)
    }
}

pub struct DecodingInfo {
    pub header: Header,
}
