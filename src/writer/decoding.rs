use std::io::Read;
use std::io;

use byteorder::{ReadBytesExt, BigEndian};

pub trait Decode {
    type DecoderState;
    fn decode<R: Read>(&mut R, &DecoderState) -> io::Result<Self>;
}

impl Decode for String {
    type DecoderState=u32;

    fn decode<R:Read>(reader: &mut R, state: u32) -> io::Result<String> {
        let len = reader.read_u16::<BigEndian>()? as usize;

    }
}
