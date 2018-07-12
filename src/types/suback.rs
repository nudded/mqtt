use super::{PacketIdentifier, ReturnCode};

#[derive(Debug)]
pub struct SubackData {
    packet_identifier: PacketIdentifier,
    return_codes: Vec<ReturnCode>
}
