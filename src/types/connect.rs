#[derive(Debug)]
pub struct ConnectData {
    keepalive: u16,
    client_identifier: String,
    clean_session: bool,
    will_topic: Option<String>,
    will_message: Option<String>,
    will_retain: bool,
    user_name: Option<String>,
    password: Option<String>,
}
