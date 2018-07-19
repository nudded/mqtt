use super::super::parser::Decode;
use super::super::parser::types::*;

use std::io::Read;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct ConnackData {
    session_present: bool,
    return_code: ConnackReturnCode
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConnackReturnCode {
    Accepted,
    UnacceptableProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
}

impl Decode for ConnackReturnCode {
    type DecoderState=();
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        match reader.read_u8()? {
            0 => Ok(ConnackReturnCode::Accepted),
            1 => Ok(ConnackReturnCode::UnacceptableProtocolVersion),
            2 => Ok(ConnackReturnCode::IdentifierRejected),
            3 => Ok(ConnackReturnCode::ServerUnavailable),
            4 => Ok(ConnackReturnCode::BadUsernameOrPassword),
            5 => Ok(ConnackReturnCode::NotAuthorized),
            _ => Err(DecodingError::Malformed),
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
        let return_code = ConnackReturnCode::decode(reader, &mut ())?;
        Ok(ConnackData { session_present, return_code })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn decoding_return_codes() {
        let mut cursor = Cursor::new(vec![0]);
        let mut return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::Accepted);

        cursor = Cursor::new(vec![1]);
        return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::UnacceptableProtocolVersion);

        cursor = Cursor::new(vec![2]);
        return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::IdentifierRejected);

        cursor = Cursor::new(vec![3]);
        return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::ServerUnavailable);

        cursor = Cursor::new(vec![4]);
        return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::BadUsernameOrPassword);

        cursor = Cursor::new(vec![5]);
        return_code = ConnackReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ConnackReturnCode::NotAuthorized);
    }

    #[test]
    fn decoding_error_while_decoding_return_codes() {
        let mut cursor = Cursor::new(vec![]);
        let result = ConnackReturnCode::decode(&mut cursor, &mut ());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decoding_connack_data_1() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,0]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, true);
        assert_eq!(connack_data.return_code, ConnackReturnCode::Accepted);
    }

    #[test]
    fn decoding_connack_data_2() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![0,2]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, false);
        assert_eq!(connack_data.return_code, ConnackReturnCode::IdentifierRejected);
    }

    #[test]
    fn decoding_connack_data_3() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,5]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, true);
        assert_eq!(connack_data.return_code, ConnackReturnCode::NotAuthorized);
    }

    #[test]
    fn decoding_connack_data_error_1() {
        let header = Header { packet_type: 2, flags: 1, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,5]);
        let result = ConnackData::decode(&mut cursor, &mut state);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decoding_connack_data_error_2() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 4};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,5]);
        let result = ConnackData::decode(&mut cursor, &mut state);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decoding_connack_data_error_3() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![2,5]);
        let result = ConnackData::decode(&mut cursor, &mut state);
        assert_eq!(result.is_err(), true);
    }
}
