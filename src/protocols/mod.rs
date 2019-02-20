mod http;
mod protocol;
mod websocket;

#[derive(PartialEq)]
pub enum State {
    Ready,
    Offline,
}

pub use self::http::Http;
pub use self::protocol::Protocol;
pub use self::websocket::Websocket;
