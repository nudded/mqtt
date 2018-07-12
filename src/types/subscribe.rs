use super::{PacketIdentifier, Qos};


#[derive(Debug)]
pub struct TopicFilter(String, Qos);

#[derive(Debug)]
pub struct SubscribeData {
    packet_identifier: PacketIdentifier,
    topic_filters: Vec<TopicFilter>
}
