use std::io::Write;
use std::io;

use byteorder::{WriteBytesExt, BigEndian};

pub trait Encode {
    /// should return the amount of bytes written, or an Error of type Err
    fn encode<W: Write>(&self, &mut W) -> io::Result<u32>;
}

impl Encode for str {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<u32> {
        let len = self.len() as u16;
        writer.write_u16::<BigEndian>(len)?;
        writer.write_all(self.as_bytes())?;
        Ok(u32::from(2+len))
    }
}

impl<T> Encode for T
    where T: AsRef<str> {

    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<u32> {
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
        let len = test_string.encode(&mut data).unwrap();
        assert_eq!(2, len);
        assert_eq!(vec![0,0], data);
    }

    #[test]
    fn it_encodes_string_objects() {
        let test_string = String::from("testing");
        let mut data: Vec<u8> = Vec::new();
        let len = test_string.encode(&mut data).unwrap();
        assert_eq!(9, len);
        let mut expected: Vec<u8> = Vec::new();
        expected.push(0);
        expected.push(7);
        expected.extend_from_slice("testing".as_bytes());
        assert_eq!(expected, data);
    }
}
