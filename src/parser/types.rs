use std::error::Error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecodingError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    MalformedHeaderError,
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

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub packet_type: u8,
    pub flags: u8,
    pub remaining_length: u32,
}

pub struct DecodingInfo {
    pub header: Header,
}
