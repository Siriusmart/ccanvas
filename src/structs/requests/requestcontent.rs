use std::path::PathBuf;

use serde::Deserialize;

use crate::structs::{Discriminator, Subscription};

use super::RenderRequest;

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
    /// add subscription to a channel with priority
    Subscribe {
        channel: Subscription,
        priority: Option<u32>,
        component: Option<Discriminator>,
    },

    #[serde(rename = "set socket")]
    /// sent responses to this socket
    SetSocket { path: PathBuf },

    #[serde(rename = "drop")]
    /// remove a single component
    Drop { discrim: Option<Discriminator> },

    #[serde(rename = "render")]
    /// render something to the terminal
    Render { content: RenderRequest },

    #[serde(rename = "spawn")]
    /// spawn a new process
    Spawn {
        command: String,
        args: Vec<String>,
        label: String,
    },
}

impl RequestContent {
    /// handle the request
    pub fn run(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
