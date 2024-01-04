use serde::{Deserialize, Serialize};
use termion::event;

/// a single mouse event
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MouseEvent {
    /// where the mouse event is
    x: u32,
    y: u32,
    /// what kind of event it is
    pub mousetype: MouseType,
}

impl From<event::MouseEvent> for MouseEvent {
    fn from(value: event::MouseEvent) -> Self {
        match value {
            event::MouseEvent::Hold(x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                mousetype: MouseType::Hold,
            },
            event::MouseEvent::Release(x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                mousetype: MouseType::Release,
            },
            event::MouseEvent::Press(mousetype, x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                mousetype: mousetype.into(),
            },
        }
    }
}

/// what kind of mouse event it is
#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MouseType {
    #[serde(rename = "left")]
    /// The left mouse button.
    Left,
    #[serde(rename = "right")]
    /// The right mouse button.
    Right,
    #[serde(rename = "middle")]
    /// The middle mouse button.
    Middle,
    #[serde(rename = "wheelup")]
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    #[serde(rename = "wheeldown")]
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
    #[serde(rename = "release")]
    /// mouse release
    Release,
    #[serde(rename = "hold")]
    /// is only emitted when u move the mouse, and only applies to left click
    Hold,
}

impl From<event::MouseButton> for MouseType {
    fn from(value: event::MouseButton) -> Self {
        match value {
            event::MouseButton::Left => Self::Left,
            event::MouseButton::Right => Self::Right,
            event::MouseButton::Middle => Self::Middle,
            event::MouseButton::WheelUp => Self::WheelUp,
            event::MouseButton::WheelDown => Self::WheelDown,
        }
    }
}
