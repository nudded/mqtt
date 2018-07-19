use super::super::parser::Decode;
use super::super::parser::types::*;

use std::io::Read;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct ConnackData {
    session_present: bool,
    return_code: ReturnCode
}

#[derive(Debug, Eq, PartialEq)]
pub enum ReturnCode {
    Accepted,
    UnacceptableProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
}

impl Decode for ReturnCode {
    type DecoderState=();
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        match reader.read_u8()? {
            0 => Ok(ReturnCode::Accepted),
            1 => Ok(ReturnCode::UnacceptableProtocolVersion),
            2 => Ok(ReturnCode::IdentifierRejected),
            3 => Ok(ReturnCode::ServerUnavailable),
            4 => Ok(ReturnCode::BadUsernameOrPassword),
            5 => Ok(ReturnCode::NotAuthorized),
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
        let return_code = ReturnCode::decode(reader, &mut ())?;
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
        let mut return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::Accepted);

        cursor = Cursor::new(vec![1]);
        return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::UnacceptableProtocolVersion);

        cursor = Cursor::new(vec![2]);
        return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::IdentifierRejected);

        cursor = Cursor::new(vec![3]);
        return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::ServerUnavailable);

        cursor = Cursor::new(vec![4]);
        return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::BadUsernameOrPassword);

        cursor = Cursor::new(vec![5]);
        return_code = ReturnCode::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(return_code, ReturnCode::NotAuthorized);
    }

    #[test]
    fn decoding_error_while_decoding_return_codes() {
        let mut cursor = Cursor::new(vec![]);
        let result = ReturnCode::decode(&mut cursor, &mut ());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decoding_connack_data_1() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,0]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, true);
        assert_eq!(connack_data.return_code, ReturnCode::Accepted);
    }

    #[test]
    fn decoding_connack_data_2() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![0,2]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, false);
        assert_eq!(connack_data.return_code, ReturnCode::IdentifierRejected);
    }

    #[test]
    fn decoding_connack_data_3() {
        let header = Header { packet_type: 2, flags: 0, remaining_length: 2};
        let mut state = DecodingInfo {header};
        let mut cursor = Cursor::new(vec![1,5]);
        let connack_data = ConnackData::decode(&mut cursor, &mut state).unwrap();
        assert_eq!(connack_data.session_present, true);
        assert_eq!(connack_data.return_code, ReturnCode::NotAuthorized);
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
