mod connect;
mod connack;
mod publish;
mod subscribe;
mod suback;
mod unsubscribe;
use self::connect::*;
use self::connack::*;
use self::publish::*;
use self::subscribe::*;
use self::suback::*;
use self::unsubscribe::*;

#[derive(Debug)]
pub struct PacketIdentifier(u16);

#[derive(Debug)]
enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

#[derive(Debug)]
enum ReturnCode {
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
