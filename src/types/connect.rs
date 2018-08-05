use std::io;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use super::*;


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
        let will_qos = Qos::decode((connect_flags & 0b0001_1000) >> 3).ok_or(DecodingError::Malformed)?;

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

impl Encode for ConnectData {
    fn encoded_length(&self) -> u32 {
        "MQTT".encoded_length() +
        self.protocol_level.encoded_length() +
        1 + // flags
        self.keepalive.encoded_length() +
        // payload
        self.client_identifier.encoded_length() +
        self.will_topic.encoded_length() +
        self.will_message.encoded_length() +
        self.user_name.encoded_length() +
        self.password.encoded_length()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        "MQTT".encode(writer)?;
        self.protocol_level.encode(writer)?;

        let mut flags = 0u8;
        if self.user_name.is_some() {flags &= 0b1000_0000}
        if self.password.is_some() {flags &= 0b0100_0000}
        if self.will_retain {flags &= 0b0010_0000}
        if self.will_retain {flags &= 0b0010_0000}
        flags &= self.will_qos.encode() << 3;
        if self.will_topic.is_some() {flags &= 0b0000_0100}
        if self.clean_session {flags &= 0b0000_0010}
        flags.encode(writer)?;

        self.keepalive.encode(writer)?;
        self.client_identifier.encode(writer)?;
        self.will_topic.encode(writer)?;
        self.will_message.encode(writer)?;
        self.user_name.encode(writer)?;
        self.password.encode(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use types::Header;

    #[test]
    fn decoding_connect_data_1() {
        let header = Header { packet_type: 1, flags: 0, remaining_length: 47};
        let mut state = DecodingInfo {header};

        let mut sample_data: Vec<u8> = vec![0,4];
        sample_data.extend_from_slice("MQTT".as_bytes());
        // protocol level
        sample_data.push(4);
        // connect flags
        sample_data.push(0b1111_0110);

        // keepalive
        sample_data.push(1);
        sample_data.push(0);

        // payload
        // client identifier
        sample_data.push(0);
        sample_data.push(4);
        sample_data.extend_from_slice("TOON".as_bytes());

        // will topic
        sample_data.push(0);
        sample_data.push(4);
        sample_data.extend_from_slice("FEBE".as_bytes());

        // will message
        sample_data.push(0);
        sample_data.push(7);
        sample_data.extend_from_slice("testing".as_bytes());

        // username
        sample_data.push(0);
        sample_data.push(6);
        sample_data.extend_from_slice("nudded".as_bytes());

        // password
        sample_data.push(0);
        sample_data.push(6);
        sample_data.extend_from_slice("nudded".as_bytes());

        let mut cursor = Cursor::new(sample_data);
        let connect_data = ConnectData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connect_data.protocol_level, 4);
        assert_eq!(connect_data.keepalive, 0b0000_0001_0000_0000);
        assert_eq!(connect_data.client_identifier, String::from("TOON"));
        assert_eq!(connect_data.clean_session, true);
        assert_eq!(connect_data.will_topic, Some(String::from("FEBE")));
        assert_eq!(connect_data.will_message, Some(String::from("testing")));
        assert_eq!(connect_data.will_retain, true);
        assert_eq!(connect_data.will_qos, Qos::ExactlyOnce);
        assert_eq!(connect_data.user_name, Some(String::from("nudded")));
        assert_eq!(connect_data.password, Some(String::from("nudded")));
    }

    #[test]
    fn decoding_connect_data_2() {
        let header = Header { packet_type: 1, flags: 0, remaining_length: 47};
        let mut state = DecodingInfo {header};

        let mut sample_data: Vec<u8> = vec![0,4];
        sample_data.extend_from_slice("MQTT".as_bytes());
        // protocol level
        sample_data.push(4);
        // connect flags
        sample_data.push(0b0000_1000);

        // keepalive
        sample_data.push(1);
        sample_data.push(0);

        // payload
        // client identifier
        sample_data.push(0);
        sample_data.push(4);
        sample_data.extend_from_slice("TOON".as_bytes());

        let mut cursor = Cursor::new(sample_data);
        let connect_data = ConnectData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connect_data.protocol_level, 4);
        assert_eq!(connect_data.keepalive, 0b0000_0001_0000_0000);
        assert_eq!(connect_data.client_identifier, String::from("TOON"));
        assert_eq!(connect_data.clean_session, false);
        assert_eq!(connect_data.will_topic, None);
        assert_eq!(connect_data.will_message, None);
        assert_eq!(connect_data.will_retain, false);
        assert_eq!(connect_data.will_qos, Qos::AtLeastOnce);
        assert_eq!(connect_data.user_name, None);
        assert_eq!(connect_data.password, None);
    }

    #[test]
    fn decoding_connect_data_3() {
        let header = Header { packet_type: 1, flags: 0, remaining_length: 47};
        let mut state = DecodingInfo {header};

        let mut sample_data: Vec<u8> = vec![0,4];
        sample_data.extend_from_slice("MQTT".as_bytes());
        // protocol level
        sample_data.push(4);
        // connect flags
        sample_data.push(0b1100_0000);

        // keepalive
        sample_data.push(1);
        sample_data.push(0);

        // payload
        // client identifier
        sample_data.push(0);
        sample_data.push(4);
        sample_data.extend_from_slice("TOON".as_bytes());

        // username
        sample_data.push(0);
        sample_data.push(6);
        sample_data.extend_from_slice("nudded".as_bytes());

        // password
        sample_data.push(0);
        sample_data.push(6);
        sample_data.extend_from_slice("nudded".as_bytes());

        let mut cursor = Cursor::new(sample_data);
        let connect_data = ConnectData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connect_data.protocol_level, 4);
        assert_eq!(connect_data.keepalive, 0b0000_0001_0000_0000);
        assert_eq!(connect_data.client_identifier, String::from("TOON"));
        assert_eq!(connect_data.clean_session, false);
        assert_eq!(connect_data.will_topic, None);
        assert_eq!(connect_data.will_message, None);
        assert_eq!(connect_data.will_retain, false);
        assert_eq!(connect_data.will_qos, Qos::AtMostOnce);
        assert_eq!(connect_data.user_name, Some(String::from("nudded")));
        assert_eq!(connect_data.password, Some(String::from("nudded")));
    }
}
