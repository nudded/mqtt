mod connect;
mod connack;
mod publish;
mod subscribe;
mod suback;
mod unsubscribe;
pub use self::connect::*;
pub use self::connack::*;
pub use self::publish::*;
pub use self::subscribe::*;
pub use self::suback::*;
pub use self::unsubscribe::*;

#[derive(Debug)]
pub struct PacketIdentifier(pub u16);

#[derive(Debug)]
pub enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

#[derive(Debug)]
pub enum ReturnCode {
    Success(Qos),
    Failure,
}

#[derive(Debug)]
pub enum Packet {
    Connect(ConnectData),
    Connack(ConnackData),
    Publish(PublishData),
    Puback(PacketIdentifier),
    Pubrec(PacketIdentifier),
    Pubrel(PacketIdentifier),
    Pubcomp(PacketIdentifier),
    Subscribe(SubscribeData),
    Suback(SubackData),
    Unsubscribe(UnsubscribeData),
    Unsuback(PacketIdentifier),
    Pingreq,
    Pingresp,
    Disconnect,
}
