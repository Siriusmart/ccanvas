use serde::Serialize;
use tokio::sync::OnceCell;

use super::ResponseContent;

static mut RESPONSE_ID: OnceCell<u32> = OnceCell::const_new_with(0);

fn resp_id() -> u32 {
    let id = unsafe { RESPONSE_ID.get_mut() }.unwrap();
    *id += 1;
    *id
}

/// a return signal back to a subprocess
#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct Response {
    /// the content of the response
    content: ResponseContent,

    /// send a confirmation to the server using this id
    /// to confirm recieved
    id: u32,

    /// request id for confirmation
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<u32>,
}

impl Response {
    pub fn new(content: ResponseContent) -> Self {
        Self {
            content,
            id: resp_id(),
            request: None,
        }
    }

    pub fn new_with_request(content: ResponseContent, request: u32) -> Self {
        Self {
            content,
            id: resp_id(),
            request: Some(request),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn content(&self) -> &ResponseContent {
        &self.content
    }
}
