use crate::structs::{Discriminator, Event, Packet, Response, ResponseContent};

use serde::Deserialize;
use tokio::sync::OnceCell;

use super::RequestContent;

static mut REQUEST_ID: OnceCell<u32> = OnceCell::const_new_with(0);

fn req_id() -> u32 {
    let id = unsafe { REQUEST_ID.get_mut() }.unwrap();
    *id += 1;
    *id
}

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
    pub fn target(&self) -> &Discriminator {
        &self.target
    }

    pub fn content(&self) -> &RequestContent {
        &self.content
    }

    pub async fn send(self) -> Response {
        let (packet, recv) = Packet::new(self);
        Event::send(Event::from_packet(packet));

        if let Ok(res) = recv.await {
            res
        } else {
            Response::new(ResponseContent::Undelivered)
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }
}
