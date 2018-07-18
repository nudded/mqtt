mod decoding;
mod types;
use self::decoding::Decode;
use self::types::*;
use super::types::*;

use std::io::Read;
use byteorder::{ReadBytesExt};

fn decode_remaining_length<R: Read>(reader: &mut R) -> Result<u32, DecodingError> {
    let mut value: u32 = 0;
    let mut multiplier: u32 = 1;
    let mut next_byte = reader.read_u8()?;
    while (next_byte & 128) != 0 {
        value += u32::from(next_byte & 127) * multiplier;
        multiplier *= 128;
        if multiplier > 128 * 128 * 128 {
            return Err(DecodingError::MalformedHeaderError);
        }
        next_byte = reader.read_u8()?;
    }
    Ok(value)
}

impl Decode for Header {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, Self::DecodingError> {
        let first_byte = reader.read_u8()?;
        let packet_type = first_byte >> 4;
        let flags = first_byte & 0b0000_1111;

        let remaining_length = decode_remaining_length(reader)?;

        let header = Header {packet_type, flags, remaining_length};

        state.header = header;
        Ok(header)
    }
}


impl Decode for Packet {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(_reader: &mut R, _state: &mut Self::DecoderState) -> Result<Self, Self::DecodingError> {
        Ok(Packet::Disconnect)
    }

}
