mod connect;
mod connack;
mod publish;
mod subscribe;
mod suback;
mod unsubscribe;
pub use self::connect::*;
pub use self::connack::*;
pub use self::publish::*;
pub use self::subscribe::*;
pub use self::suback::*;
pub use self::unsubscribe::*;

use super::parser::Decode;
use super::parser::types::*;

use std::io::Read;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct PacketIdentifier(pub u16);

#[derive(Debug, Eq, PartialEq)]
pub enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

impl Qos {
    fn decode(bits: u8) -> Option<Qos> {
        match bits {
            0 => Some(Qos::AtMostOnce),
            1 => Some(Qos::AtMostOnce),
            2 => Some(Qos::ExactlyOnce),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ReturnCode {
    Success(Qos),
    Failure,
}

impl Decode for ReturnCode {
    type DecoderState=DecodingInfo;
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
