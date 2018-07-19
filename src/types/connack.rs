use super::super::parser::Decode;
use super::super::parser::types::*;

use std::io::Read;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct ConnackData {
    session_present: bool,
    return_code: ReturnCode
}

#[derive(Debug)]
pub enum ReturnCode {
    Accepted,
    UnacceptableProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
}

impl Decode for ReturnCode {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        match reader.read_u8()? {
            0 => Ok(ReturnCode::Accepted),
            1 => Ok(ReturnCode::UnacceptableProtocolVersion),
            2 => Ok(ReturnCode::IdentifierRejected),
            3 => Ok(ReturnCode::ServerUnavailable),
            4 => Ok(ReturnCode::BadUsernameOrPassword),
            5 => Ok(ReturnCode::NotAuthorized),
            _ => Err(DecodingError::Forbidden),
        }
    }
}

impl Decode for ConnackData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        if state.header.flags != 0 { return Err(DecodingError::Malformed) };
        if state.header.remaining_length != 2 { return Err(DecodingError::Malformed) };

        let first_byte = reader.read_u8()?;
        if first_byte > 1 { return Err(DecodingError::Malformed) };

        let session_present = first_byte == 1;

        let return_code = ReturnCode::decode(reader, state)?;
        Ok(ConnackData { session_present, return_code })
    }
}


