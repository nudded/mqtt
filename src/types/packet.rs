use super::*;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::io::{Read, Write};
use std::io;

#[derive(Debug)]
pub struct PacketIdentifier(pub u16);

#[derive(Debug, Eq, PartialEq)]
pub enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub packet_type: u8,
    pub flags: u8,
    pub remaining_length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReturnCode {
    Success(Qos),
    Failure,
}

#[derive(Debug)]
pub enum Packet {
    Connect(ConnectData),
    Connack(ConnackData),
    Publish(PublishData),
    Puback(PacketIdentifier),
    Pubrec(PacketIdentifier),
    Pubrel(PacketIdentifier),
    Pubcomp(PacketIdentifier),
    Subscribe(SubscribeData),
    Suback(SubackData),
    Unsubscribe(UnsubscribeData),
    Unsuback(PacketIdentifier),
    Pingreq,
    Pingresp,
    Disconnect,
}

impl Qos {
    pub fn decode(bits: u8) -> Option<Qos> {
        match bits {
            0 => Some(Qos::AtMostOnce),
            1 => Some(Qos::AtLeastOnce),
            2 => Some(Qos::ExactlyOnce),
            _ => None,
        }
    }
    pub fn encode(&self) -> u8 {
        match self {
            Qos::AtMostOnce => 0,
            Qos::AtLeastOnce => 1,
            Qos::ExactlyOnce => 2
        }
    }
}

impl Decode for ReturnCode {
    type DecoderState=();
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        let byte = reader.read_u8()?;
        match byte {
            0x00 => Ok(ReturnCode::Success(Qos::AtMostOnce)),
            0x01 => Ok(ReturnCode::Success(Qos::AtLeastOnce)),
            0x02 => Ok(ReturnCode::Success(Qos::ExactlyOnce)),
            0x80 => Ok(ReturnCode::Failure),
            _ => Err(DecodingError::Malformed),
        }
    }
}

impl Encode for ReturnCode {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            ReturnCode::Success(qos) => writer.write_u8(qos.encode()),
            ReturnCode::Failure => writer.write_u8(0x80),
        }
    }
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

impl Encode for Header {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8((self.packet_type << 4) | self.flags)?;
        encode_remaining_length(self.remaining_length, writer)
    }
}

impl Header {
    fn for_packet(packet: &Packet) -> Header {
        match packet {
            Packet::Connect(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 1},
            Packet::Connack(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 2},
            Packet::Publish(data) => Header {flags: data.flags(), remaining_length: data.encoded_length(), packet_type: 3},
            Packet::Puback(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 4},
            Packet::Pubrec(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 5},
            Packet::Pubrel(data) => Header {flags: 2, remaining_length: data.encoded_length(), packet_type: 6},
            Packet::Pubcomp(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 7},
            Packet::Subscribe(data) => Header {flags: 2, remaining_length: data.encoded_length(), packet_type: 8},
            Packet::Suback(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 9},
            Packet::Unsubscribe(data) => Header {flags: 2, remaining_length: data.encoded_length(), packet_type: 10},
            Packet::Unsuback(data) => Header {flags: 0, remaining_length: data.encoded_length(), packet_type: 11},
            Packet::Pingreq => Header {flags: 0, remaining_length: 0, packet_type: 12},
            Packet::Pingresp => Header {flags: 0, remaining_length: 0, packet_type: 13},
            Packet::Disconnect => Header {flags: 0, remaining_length: 0, packet_type: 14},
        }
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

impl Encode for PacketIdentifier {
    fn encoded_length(&self) -> u32 {2}
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u16::<BigEndian>(self.0)
    }
}

fn encode_remaining_length<W: Write>(remaining_length: u32, writer: &mut W) -> io::Result<()> {
    let mut encoded_byte;
    let mut value = remaining_length;

    while value > 0 {
        encoded_byte = remaining_length % 128;
        value = remaining_length / 128;
        if value > 0 {
            encoded_byte |= 128;
        }
        writer.write_u8(encoded_byte as u8)?;
    }
    Ok(())
}

fn decode_remaining_length<R: Read>(reader: &mut R) -> Result<u32, DecodingError> {
    let mut value: u32 = 0;
    let mut multiplier: u32 = 1;
    let mut next_byte = reader.read_u8()?;
    while (next_byte & 128) != 0 {
        value += u32::from(next_byte & 127) * multiplier;
        multiplier *= 128;
        if multiplier > 128 * 128 * 128 {
            return Err(DecodingError::Malformed);
        }
        next_byte = reader.read_u8()?;
    }
    Ok(value)
}

impl Encode for Packet {

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let header = Header::for_packet(self);
        match self {
            Packet::Connect(data) => Packet::encode_with_header(writer, data, header),
            Packet::Connack(data) => Packet::encode_with_header(writer, data, header),
            Packet::Publish(data) => Packet::encode_with_header(writer, data, header),
            Packet::Puback(data) => Packet::encode_with_header(writer, data, header),
            Packet::Pubrec(data) => Packet::encode_with_header(writer, data, header),
            Packet::Pubrel(data) => Packet::encode_with_header(writer, data, header),
            Packet::Pubcomp(data) => Packet::encode_with_header(writer, data, header),
            Packet::Subscribe(data) => Packet::encode_with_header(writer, data, header),
            Packet::Suback(data) => Packet::encode_with_header(writer, data, header),
            Packet::Unsubscribe(data) => Packet::encode_with_header(writer, data, header),
            Packet::Unsuback(data) => Packet::encode_with_header(writer, data, header),
            Packet::Pingreq => header.encode(writer),
            Packet::Pingresp => header.encode(writer),
            Packet::Disconnect => header.encode(writer),
        }
    }
}

impl Packet {

    fn encode_with_header<W: Write, E: Encode>(writer: &mut W, encodable: &E, header: Header) -> io::Result<()> {
        header.encode(writer)?;
        encodable.encode(writer)
    }

    fn decode_connect<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = ConnectData::decode(reader, state)?;
        Ok(Packet::Connect(data))
    }
    fn decode_connack<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = ConnackData::decode(reader, state)?;
        Ok(Packet::Connack(data))
    }
    fn decode_publish<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = PublishData::decode(reader, state)?;
        Ok(Packet::Publish(data))
    }
    fn decode_subscribe<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = SubscribeData::decode(reader, state)?;
        Ok(Packet::Subscribe(data))
    }
    fn decode_suback<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = SubackData::decode(reader, state)?;
        Ok(Packet::Suback(data))
    }
    fn decode_unsubscribe<R: Read>(reader: &mut R, state: &mut DecodingInfo) -> Result<Self, DecodingError> {
        let data = UnsubscribeData::decode(reader, state)?;
        Ok(Packet::Unsubscribe(data))
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
    use std::io::Cursor;

    #[test]
    fn decoding_qos() {
        assert_eq!(Qos::decode(0), Some(Qos::AtMostOnce));
        assert_eq!(Qos::decode(1), Some(Qos::AtLeastOnce));
        assert_eq!(Qos::decode(2), Some(Qos::ExactlyOnce));
        assert_eq!(Qos::decode(80), None);
    }

    #[test]
    fn encoding_qos() {
        assert_eq!(Qos::AtMostOnce.encode(), 0);
        assert_eq!(Qos::AtLeastOnce.encode(), 1);
        assert_eq!(Qos::ExactlyOnce.encode(), 2);
    }

    #[test]
    fn decoding_return_codes() {
        let mut data = Cursor::new(vec![0]);
        assert_eq!(ReturnCode::decode(&mut data, &mut ()).unwrap(), ReturnCode::Success(Qos::AtMostOnce));

        data = Cursor::new(vec![1]);
        assert_eq!(ReturnCode::decode(&mut data, &mut ()).unwrap(), ReturnCode::Success(Qos::AtLeastOnce));

        data = Cursor::new(vec![2]);
        assert_eq!(ReturnCode::decode(&mut data, &mut ()).unwrap(), ReturnCode::Success(Qos::ExactlyOnce));

        data = Cursor::new(vec![0x80]);
        assert_eq!(ReturnCode::decode(&mut data, &mut ()).unwrap(), ReturnCode::Failure);

        data = Cursor::new(vec![10]);
        assert_eq!(ReturnCode::decode(&mut data, &mut ()).is_err(), true);
    }

    #[test]
    fn encoding_return_codes() {
        let mut data: Vec<u8> = Vec::new();
        ReturnCode::Success(Qos::AtMostOnce).encode(&mut data).unwrap();
        assert_eq!(data, vec![0]);
        data.clear();

        ReturnCode::Success(Qos::AtLeastOnce).encode(&mut data).unwrap();
        assert_eq!(data, vec![1]);
        data.clear();
        ReturnCode::Success(Qos::ExactlyOnce).encode(&mut data).unwrap();
        assert_eq!(data, vec![2]);
        data.clear();

        ReturnCode::Failure.encode(&mut data).unwrap();
        assert_eq!(data, vec![0x80]);
    }
}

