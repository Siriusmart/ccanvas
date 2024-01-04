use crate::structs::{Discriminator, Packet, Request, Response, Subscription};

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
    /// message sent from a component
    Focus,
    Unfocus,
    Message {
        sender: Discriminator,
        target: Discriminator,
        content: String,
    },
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

impl Clone for Event {
    fn clone(&self) -> Self {
        match self {
            Self::KeyPress(key) => Self::KeyPress(*key),
            Self::MouseEvent(mouse) => Self::MouseEvent(*mouse),
            Self::ScreenResize(x, y) => Self::ScreenResize(*x, *y),
            Self::Focus => Self::Focus,
            Self::Unfocus => Self::Unfocus,
            Self::Message {
                sender,
                target,
                content,
            } => Self::Message {
                sender: sender.clone(),
                target: target.clone(),
                content: content.clone(),
            },
            Self::RequestPacket(_) => panic!("bad clone"),
        }
    }
}

impl Event {
    pub fn subscriptions(&self) -> Vec<Subscription> {
        match self {
            Self::KeyPress(key) => vec![
                Subscription::AllKeyPresses,
                Subscription::SpecificKeyPress { key: *key },
                Subscription::SpecificKeyCode { code: key.code },
                Subscription::SpecificKeyModifier {
                    modifier: key.modifier,
                },
            ],
            Self::Message { sender, .. } => vec![
                Subscription::AllMessages,
                Subscription::SpecificMessage {
                    source: sender.clone(),
                },
            ],
            Self::MouseEvent(mouse) => vec![
                Subscription::AllMouseEvents,
                Subscription::SpecificMouseEvent {
                    mouse: mouse.mousetype,
                },
            ],
            Self::ScreenResize(..) => vec![Subscription::ScreenResize],
            Self::Focus { .. } => vec![Subscription::Focused],
            Self::Unfocus => vec![Subscription::Unfocused],
            Self::RequestPacket(_) => Vec::new(),
        }
    }

    pub fn from_packet(packet: Packet<Request, Response>) -> Self {
        Self::RequestPacket(packet)
    }
}
