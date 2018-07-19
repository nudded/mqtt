use super::super::parser::Decode;
use super::super::parser::types::*;
use super::Qos;
use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian};


#[derive(Debug)]
pub struct ConnectData {
    protocol_level: u8,
    keepalive: u16,
    client_identifier: String,
    clean_session: bool,
    will_topic: Option<String>,
    will_message: Option<String>,
    will_retain: bool,
    will_qos: Qos,
    user_name: Option<String>,
    password: Option<String>,
}

impl Decode for ConnectData {
    type DecoderState=DecodingInfo;
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, state: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        if state.header.flags != 0 { return Err(DecodingError::Malformed) };

        let protocol_name = String::decode(reader, &mut ())?;
        if protocol_name != "MQTT" { return Err(DecodingError::Malformed) };

        // this should equal 4, but has to be handled in the response, so no error here
        let protocol_level = reader.read_u8()?;
        let connect_flags = reader.read_u8()?;
        // validate that the first bit is set to zero, otherwise this must be an error
        if connect_flags & 1 > 0 { return Err(DecodingError::Malformed) };

        let clean_session = connect_flags & 0b0000_0010 > 0;
        let keepalive = reader.read_u16::<BigEndian>()?;

        let client_identifier = String::decode(reader, &mut ())?;

        // these will be instantiated later on
        let will_topic;
        let will_message;
        let user_name;
        let password;

        // check will_flag
        if connect_flags & 0b0000_0100 > 0 {
            will_topic = Some(String::decode(reader, &mut ())?);
            will_message = Some(String::decode(reader, &mut ())?);
        } else {
            will_topic = None;
            will_message= None;
        }

        // check user_name flag
        if connect_flags & 0b1000_0000 > 0 {
            user_name = Some(String::decode(reader, &mut ())?);
        } else {
            user_name = None;
        }

        // check password flag
        if connect_flags & 0b0100_0000 > 0 {
            password = Some(String::decode(reader, &mut ())?);
        } else {
            password = None;
        }

        let will_retain = connect_flags & 0b0010_0000 > 0;

        // this is a sort-of decode method that does not actually decode like this method does
        let will_qos = Qos::decode(connect_flags & 0b0001_1000 >> 3).ok_or(DecodingError::Malformed)?;

        Ok(ConnectData {
            protocol_level,
            keepalive,
            client_identifier,
            clean_session,
            will_topic,
            will_message,
            will_retain,
            will_qos,
            user_name,
            password})
    }
}
