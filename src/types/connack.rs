#[derive(Debug)]
pub struct ConnackData {
    session_present: bool,
    return_code: ReturnCode
}

#[derive(Debug)]
enum ReturnCode {
    ConnectionAccepted,
    ConnectionRefused(String),
}

