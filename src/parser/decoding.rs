use super::types::DecodingError;
use std::io::Read;
use std::error::Error;

use byteorder::{ReadBytesExt, BigEndian};

pub trait Decode: Sized {
    type DecoderState;
    type DecodingError: Error;

    fn decode<R: Read>(&mut R, &mut Self::DecoderState) -> Result<Self, Self::DecodingError>;
}

impl Decode for String {
    type DecoderState=();
    type DecodingError=DecodingError;

    fn decode<R: Read>(reader: &mut R, _: &mut Self::DecoderState) -> Result<Self, DecodingError> {
        let len = reader.read_u16::<BigEndian>().unwrap();
        let mut buf = Vec::with_capacity(len as usize);

        reader.take(len as u64).read_to_end(&mut buf)?;
        String::from_utf8(buf).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn it_decodes_empty_string_values() {
        let mut cursor = Cursor::new(vec![0,0]);
        let decoded_string = String::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(decoded_string, String::new());
    }

    #[test]
    fn it_decodes_string_values() {
        let mut data: Vec<u8> = vec![0,7];
        data.extend_from_slice("testing".as_bytes());
        let mut cursor = Cursor::new(data);
        let decoded_string = String::decode(&mut cursor, &mut ()).unwrap();
        assert_eq!(decoded_string, String::from("testing"));
    }
}
