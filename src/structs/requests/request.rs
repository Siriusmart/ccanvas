use serde::Deserialize;

use crate::structs::Discriminator;

use super::RequestContent;

/// a signal that comes from a subprocess
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Request {
    /// reciever
    target: Discriminator,
    /// the content of the request
    #[serde(flatten)]
    content: RequestContent,
}

impl Request {
    pub fn target(&self) -> &Discriminator {
        &self.target
    }

    pub fn content(&self) -> &RequestContent {
        &self.content
    }
}
