use std::process::Command;

use crate::structs::{Event, KeyEvent};

use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum EventSerde {
    /// keyboard event
    #[serde(rename = "key")]
    Key(KeyEvent),
    /// screen resize event (should trigger a rerender)
    #[serde(rename = "resize")]
    Resize { width: u32, height: u32 },
}

impl EventSerde {
    pub fn from_event(value: &Event) -> Self {
        match value {
            Event::KeyPress(key) => Self::Key(key.clone()),
            Event::ScreenResize(width, height) => Self::Resize {
                width: *width,
                height: *height,
            },
            Event::RequestPacket(_) | Event::RegSubscription(_, _, _) => {
                unreachable!("should not happend")
            }
        }
    }
}
