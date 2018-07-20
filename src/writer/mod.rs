mod encoding;

use self::encoding::Encode;
use super::types::*;

use std::io::Write;
use std::io;
use byteorder::{WriteBytesExt, BigEndian};

fn encode_remaining_length<W: Write>(remaining_length: u32, writer: &mut W) -> io::Result<()> {
    let mut encoded_byte;
    let mut value = remaining_length;

    while value > 0 {
        encoded_byte = remaining_length % 128;
        value = remaining_length / 128;
        if value > 0 {
            encoded_byte |= 128;
        }
        writer.write_u8(encoded_byte as u8)?;
    }
    Ok(())
}

impl Encode for Header {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8((self.packet_type << 4) | self.flags)?;
        encode_remaining_length(self.remaining_length, writer)
    }
}

impl Encode for PacketIdentifier {
    fn encoded_length(&self) -> u32 {2}
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u16::<BigEndian>(self.0)
    }
}

