mod decoding;
mod types;
use self::decoding::Decode;
use self::types::*;
use super::types::*;

use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian};

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

impl Decode for PacketIdentifier {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _state: &mut Self::DecoderState) -> Result<Self, Self::DecodingError> {
        let value = reader.read_u16::<BigEndian>()?;
        Ok(PacketIdentifier(value))
    }
}

impl Packet {
    fn decode_connect<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
    fn decode_connack<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
    fn decode_publish<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
    fn decode_subscribe<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
    fn decode_suback<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
    fn decode_unsubscribe<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        Ok(Packet::Disconnect)
    }
}

impl Decode for Packet {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, Self::DecodingError> {
        let header = Header::decode(reader, state)?;
        match header.packet_type {
            1 => Packet::decode_connect(reader, state),
            2 => Packet::decode_connack(reader, state),
            3 => Packet::decode_publish(reader, state),
            4 => Ok(Packet::Puback(PacketIdentifier::decode(reader, state)?)),
            5 => Ok(Packet::Pubrec(PacketIdentifier::decode(reader, state)?)),
            6 => Ok(Packet::Pubrel(PacketIdentifier::decode(reader, state)?)),
            7 => Ok(Packet::Pubcomp(PacketIdentifier::decode(reader, state)?)),
            8 => Packet::decode_subscribe(reader, state),
            9 => Packet::decode_suback(reader, state),
            10 => Packet::decode_unsubscribe(reader, state),
            11 => Ok(Packet::Unsuback(PacketIdentifier::decode(reader, state)?)),
            12 => Ok(Packet::Pingreq),
            13 => Ok(Packet::Pingresp),
            14 => Ok(Packet::Disconnect),
            _ => Err(DecodingError::Forbidden),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
