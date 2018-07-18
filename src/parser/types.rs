use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DecodingError {
    message: String,
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoding failed: {}", self.message)
    }
}

impl Error for DecodingError {
}

struct Header {
    packet_type: u8,
    flags: u8,
    remaining_length: u32,
}

pub struct DecodingInfo {
    header: Header,
}
