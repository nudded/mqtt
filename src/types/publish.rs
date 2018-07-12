use super::{PacketIdentifier, Qos};

#[derive(Debug)]
pub struct PublishData {
    qos: Qos,
    retain: bool,
    dup: bool,
    packet_identifier: Option<PacketIdentifier>,
    topic_name: String,
    message: String,
}
