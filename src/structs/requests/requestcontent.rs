use std::path::PathBuf;

use serde::Deserialize;

use crate::structs::Subscription;

/// variations of requests
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum RequestContent {
    #[serde(rename = "confirm recieve")]
    /// confirm that an event has been recieved
    ConfirmRecieve {
        /// event id
        id: u32,
        /// true = does not capture event
        pass: bool,
    },

    #[serde(rename = "subscribe")]
    Subscribe {
        channel: Subscription,
        priority: Option<u32>,
    },

    #[serde(rename = "set socket")]
    SetSocket { path: PathBuf },
}

impl RequestContent {
    /// handle the request
    pub fn run(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
