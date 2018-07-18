use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DecodingError {
    message: String,
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.message.fmt(&mut f)
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
