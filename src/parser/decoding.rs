use std::io::Read;
use std::error::Error;
use std::string::FromUtf8Error;

use byteorder::{ReadBytesExt, BigEndian};

pub trait Decode: Sized {
    type DecoderState;
    type DecodingError: Error;

    fn decode<R: Read>(&mut R, &mut Self::DecoderState) -> Result<Self, Self::DecodingError>;
}

impl Decode for String {
    type DecoderState=();
    type DecodingError=FromUtf8Error;

    fn decode<R: Read>(reader: &mut R, _: &mut Self::DecoderState) -> Result<Self, FromUtf8Error> {
        let len = reader.read_u16::<BigEndian>().unwrap();
        let mut buf = Vec::with_capacity(len as usize);
        reader.read_exact(&mut buf).expect("Could not read expected amount of bytes");
        String::from_utf8(buf)
    }
}

