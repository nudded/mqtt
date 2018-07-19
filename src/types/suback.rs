use super::{PacketIdentifier, ReturnCode};
use super::super::parser::Decode;
use super::super::parser::types::*;
use std::io::Read;

#[derive(Debug)]
pub struct SubackData {
    packet_identifier: PacketIdentifier,
    return_codes: Vec<ReturnCode>
}

impl Decode for SubackData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        if state.header.flags != 0 { return Err(DecodingError::Malformed) };
        let packet_identifier = PacketIdentifier::decode(reader, state)?;

        let mut remaining_bytes = state.header.remaining_length - 2;
        let mut return_codes = Vec::with_capacity(remaining_bytes as usize);

        while remaining_bytes > 0 {
            return_codes.push(ReturnCode::decode(reader, state)?);
            remaining_bytes -= 1;
        }

        Ok(SubackData { packet_identifier, return_codes })
    }
}
