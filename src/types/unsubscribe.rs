use super::{PacketIdentifier};

#[derive(Debug)]
pub struct UnsubscribeData {
    packet_identifier: PacketIdentifier,
    topic_filters: Vec<String>
}
