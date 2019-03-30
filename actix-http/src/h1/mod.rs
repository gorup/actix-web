//! HTTP/1 implementation
use bytes::Bytes;

mod client;
mod codec;
mod decoder;
mod dispatcher;
mod encoder;
mod payload;
mod service;

pub use self::client::{ClientCodec, ClientPayloadCodec};
pub use self::codec::Codec;
pub use self::dispatcher::Dispatcher;
pub use self::payload::{Payload, PayloadBuffer};
pub use self::service::{H1Service, H1ServiceHandler, OneRequest};

#[derive(Debug)]
/// Codec message
pub enum Message<T> {
    /// Http message
    Item(T),
    /// Payload chunk
    Chunk(Option<Bytes>),
}

impl<T> From<T> for Message<T> {
    fn from(item: T) -> Self {
        Message::Item(item)
    }
}

/// Incoming request type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    None,
    Payload,
    Stream,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;

    impl Message<Request> {
        pub fn message(self) -> Request {
            match self {
                Message::Item(req) => req,
                _ => panic!("error"),
            }
        }

        pub fn chunk(self) -> Bytes {
            match self {
                Message::Chunk(Some(data)) => data,
                _ => panic!("error"),
            }
        }

        pub fn eof(self) -> bool {
            match self {
                Message::Chunk(None) => true,
                Message::Chunk(Some(_)) => false,
                _ => panic!("error"),
            }
        }
    }
}