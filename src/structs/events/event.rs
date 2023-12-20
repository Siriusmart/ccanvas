use super::KeyEvent;

use termion::event::Event as TermionEvent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    KeyPress(KeyEvent),
    ScreenResize(u32, u32),
    Unknown(String),
}

impl TryFrom<TermionEvent> for Event {
    fn try_from(value: TermionEvent) -> Result<Self, Self::Error> {
        match value {
            TermionEvent::Key(keyevent) => Ok(Self::KeyPress(KeyEvent::try_from(keyevent)?)),
            TermionEvent::Mouse(mouseevent) => todo!(),
            TermionEvent::Unsupported(bytes) => Err(crate::Error::UnsupportedEvent(bytes)),
        }
    }

    type Error = crate::Error;
}
