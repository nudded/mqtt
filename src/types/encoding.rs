use std::io::Write;
use std::io;

use byteorder::{WriteBytesExt, BigEndian};

pub trait Encode {
    /// used to calculate the remaining_length in the header field
    fn encoded_length(&self) -> u32 {0}

    /// encode the mqtt data onto the writer
    fn encode<W: Write>(&self, &mut W) -> io::Result<()>;
}

impl Encode for str {
    fn encoded_length(&self) -> u32 {
        self.len() as u32 + 2
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let len = self.len() as u16;
        writer.write_u16::<BigEndian>(len)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T> Encode for T
    where T: AsRef<str> {
    fn encoded_length(&self) -> u32 {
        self.as_ref().encoded_length()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.as_ref().encode(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_empty_string_values() {
        let test_string = "";
        let mut data: Vec<u8> = Vec::new();
        test_string.encode(&mut data).unwrap();
        assert_eq!(vec![0,0], data);
        assert_eq!(test_string.encoded_length(), 2);
    }

    #[test]
    fn it_encodes_string_objects() {
        let test_string = String::from("testing");
        let mut data: Vec<u8> = Vec::new();
        test_string.encode(&mut data).unwrap();
        let mut expected: Vec<u8> = Vec::new();
        expected.push(0);
        expected.push(7);
        expected.extend_from_slice("testing".as_bytes());
        assert_eq!(expected, data);
        assert_eq!(test_string.encoded_length(), 9);
    }
}
