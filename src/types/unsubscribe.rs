use super::PacketIdentifier;
use super::super::parser::Decode;
use super::super::parser::types::*;
use std::io::Read;

#[derive(Debug)]
pub struct UnsubscribeData {
    packet_identifier: PacketIdentifier,
    topic_filters: Vec<String>
}

impl Decode for UnsubscribeData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        if state.header.flags != 2 {return Err(DecodingError::Malformed)};

        let packet_identifier = PacketIdentifier::decode(reader, state)?;
        let mut remaining_length = state.header.remaining_length - 2;
        let mut topic_filters = Vec::new();

        while remaining_length > 0 {
            let filter = String::decode(reader, &mut ())?;
            remaining_length -= 2 + filter.len() as u32;
            topic_filters.push(filter);
        }

        Ok(UnsubscribeData {packet_identifier, topic_filters})
    }
}
