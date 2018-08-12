use super::*;
use std::io;
use std::io::{Write, Read};

#[derive(Debug)]
pub struct PublishData {
    qos: Qos,
    retain: bool,
    dup: bool,
    packet_identifier: Option<PacketIdentifier>,
    topic_name: String,
    message: String,
}

impl Decode for PublishData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        let flags = state.header.flags;
        let dup = flags & 0b0000_1000 > 0;
        let retain = flags & 0b0000_0001 > 0;
        let qos = Qos::decode((flags & 0b0000_0110) >> 1).ok_or(DecodingError::Malformed)?;

        let topic_name = String::decode(reader, &mut ())?;
        let packet_identifier = if qos == Qos::AtLeastOnce || qos == Qos::ExactlyOnce {
            Some(PacketIdentifier::decode(reader, state)?)
        } else {
            None
        };
        let message = String::decode(reader, &mut ())?;
        Ok(PublishData { qos, retain, dup, packet_identifier, topic_name, message})
    }
}

impl Encode for PublishData {

    fn encoded_length(&self) -> u32 {
        self.topic_name.encoded_length() +
        self.packet_identifier.encoded_length() +
        self.message.encoded_length()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.topic_name.encode(writer)?;
        self.packet_identifier.encode(writer)?;
        self.message.encode(writer)
    }
}
impl PublishData {
    pub fn flags(&self) -> u8 {
        let mut flags = 0u8;
        if self.dup { flags &= 0b0000_1000 };
        if self.retain { flags &= 1 };
        flags &= self.qos.encode() << 1;
        flags
    }
}
