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
    fn decode(bits: u8) -> Option<Qos> {
        match bits {
            0 => Some(Qos::AtMostOnce),
            1 => Some(Qos::AtLeastOnce),
            2 => Some(Qos::ExactlyOnce),
            _ => None,
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
}
