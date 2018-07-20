use super::*;
use std::io::Read;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct TopicFilter {
    filter: String,
    qos: Qos
}

impl TopicFilter {
    fn len(&self) -> usize {
        self.filter.bytes().len() + 2 + 1
    }
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
            remaining_length -= filter.len() as u32;
            topic_filters.push(filter)
        }

        Ok(SubscribeData { packet_identifier, topic_filters})
    }
}
