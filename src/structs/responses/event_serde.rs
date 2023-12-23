use crate::structs::{Event, KeyEvent, MouseEvent};

use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum EventSerde {
    /// keyboard event
    #[serde(rename = "key")]
    Key(KeyEvent),
    #[serde(rename = "mouse")]
    Mouse(MouseEvent),
    /// screen resize event (should trigger a rerender)
    #[serde(rename = "resize")]
    Resize { width: u32, height: u32 },
}

impl EventSerde {
    pub fn from_event(value: &Event) -> Self {
        match value {
            Event::KeyPress(key) => Self::Key(*key),
            Event::ScreenResize(width, height) => Self::Resize {
                width: *width,
                height: *height,
            },
            Event::MouseEvent(mouse) => Self::Mouse(*mouse),
            Event::RequestPacket(_) => unreachable!("should not happend"),
        }
    }
}
