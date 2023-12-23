use crate::structs::{Packet, Request, Response, Subscription};

use super::{KeyEvent, MouseEvent};

use termion::event::Event as TermionEvent;

/// a basic, generic unit of event
#[derive(Debug, PartialEq)]
pub enum Event {
    /// keyboard event
    KeyPress(KeyEvent),
    /// events related to mouse down
    MouseEvent(MouseEvent),
    /// screen resize event (should trigger a rerender)
    ScreenResize(u32, u32),
    /// request that requires a response
    RequestPacket(Packet<Request, Response>),
}

impl TryFrom<TermionEvent> for Event {
    fn try_from(value: TermionEvent) -> Result<Self, Self::Error> {
        match value {
            TermionEvent::Key(keyevent) => Ok(Self::KeyPress(KeyEvent::try_from(keyevent)?)),
            TermionEvent::Mouse(mouseevent) => Ok(Self::MouseEvent(MouseEvent::from(mouseevent))),
            TermionEvent::Unsupported(bytes) => Err(crate::Error::UnsupportedEvent(bytes)),
        }
    }

    type Error = crate::Error;
}

impl Event {
    pub fn subscriptions(&self) -> &[Subscription] {
        match self {
            Self::KeyPress(_keyevent) => &[Subscription::AllKeyPresses],
            _ => &[],
        }
    }

    pub fn from_packet(packet: Packet<Request, Response>) -> Self {
        Self::RequestPacket(packet)
    }
}
