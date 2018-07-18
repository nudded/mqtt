mod decoding;
mod types;
use self::decoding::Decode;
use self::types::*;
use super::types::*;

use std::io::Read;

impl Decode for Packet {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, Self::DecodingError> {
        Ok(Packet::Disconnect)
    }

}
