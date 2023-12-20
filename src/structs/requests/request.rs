use serde::Deserialize;

use super::RequestContent;

/// a signal that comes from a subprocess
#[derive(Deserialize)]
pub struct Request {
    /// sender
    source: Vec<u32>,
    /// reciever
    target: Vec<u32>,
    /// the content of the request
    content: RequestContent,
}
