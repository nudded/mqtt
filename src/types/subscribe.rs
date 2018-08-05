use super::*;
use std::io;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct TopicFilter {
    filter: String,
    qos: Qos
}

impl Decode for TopicFilter {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        let filter = String::decode(reader, &mut ())?;

        let qos_byte = reader.read_u8()?;
        if qos_byte > 3 { return Err(DecodingError::Malformed) };
        let qos = Qos::decode(qos_byte).ok_or(DecodingError::Malformed)?;

        Ok(TopicFilter {filter, qos})
    }
}

impl Encode for TopicFilter {
    fn encoded_length(&self) -> u32 {
        self.filter.encoded_length() + 1
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.filter.encode(writer)?;
        writer.write_u8(self.qos.encode())
    }
}

#[derive(Debug)]
pub struct SubscribeData {
    packet_identifier: PacketIdentifier,
    topic_filters: Vec<TopicFilter>
}

impl Decode for SubscribeData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        if state.header.flags != 2 {return Err(DecodingError::Malformed)};

        let packet_identifier = PacketIdentifier::decode(reader, state)?;
        let mut topic_filters = Vec::new();

        let mut remaining_length = state.header.remaining_length - 2;
        while remaining_length > 0 {
            let filter = TopicFilter::decode(reader, state)?;
            remaining_length -= filter.encoded_length();
            topic_filters.push(filter)
        }

        Ok(SubscribeData { packet_identifier, topic_filters})
    }
}

impl Encode for SubscribeData {
    fn encoded_length(&self) -> u32 {
        self.packet_identifier.encoded_length() +
        self.topic_filters.encoded_length()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.packet_identifier.encode(writer)?;
        self.topic_filters.encode(writer)
    }
}
