use std::path::PathBuf;

use serde::Serialize;

use super::{EventSerde, ResponseError, ResponseSuccess};

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseContent {
    #[serde(rename = "undelivered")]
    Undelivered,

    #[serde(rename = "event")]
    Event { content: EventSerde },

    #[serde(rename = "error")]
    Error { content: ResponseError },

    #[serde(rename = "success")]
    Success { content: ResponseSuccess },

    /// will not recieve this
    #[serde(rename = "set socket")]
    SetSocket(PathBuf),
}
