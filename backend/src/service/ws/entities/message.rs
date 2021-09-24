use crate::service::ws::entities::SessionId;
use actix::Message;
use bytes::Bytes;
use std::fmt::Formatter;

#[derive(Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct WsMessageAdaptor(pub Bytes);

impl std::ops::Deref for WsMessageAdaptor {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target { &self.0 }
}
