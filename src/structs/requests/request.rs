use crate::structs::{Discriminator, Event, Packet, Response, ResponseContent};

use super::RequestContent;
use serde::Deserialize;

/// a signal that comes from a subprocess
#[derive(Deserialize, Debug)]
pub struct Request {
    /// reciever
    target: Discriminator,
    /// the content of the request
    content: RequestContent,
    /// confirmation identifier
    id: u32,
}

impl Request {
    /// returns discrim of target component
    pub fn target(&self) -> &Discriminator {
        &self.target
    }

    /// returns RequestContent
    pub fn content(&self) -> &RequestContent {
        &self.content
    }

    /// send self to master space, and wait for response
    pub async fn send(self) -> Response {
        let (packet, recv) = Packet::new(self);
        Event::send(Event::from_packet(packet));

        if let Ok(res) = recv.await {
            res
        } else {
            Response::new(ResponseContent::Undelivered)
        }
    }

    /// get request id
    pub fn id(&self) -> &u32 {
        &self.id
    }
}
