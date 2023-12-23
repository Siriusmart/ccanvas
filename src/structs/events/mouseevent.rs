use serde::Serialize;
use termion::event;

/// a single mouse event
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MouseEvent {
    /// where the mouse event is
    x: u32,
    y: u32,
    #[serde(rename = "type")]
    /// what kind of event it is
    r#type: MouseType,
}

impl From<event::MouseEvent> for MouseEvent {
    fn from(value: event::MouseEvent) -> Self {
        match value {
            event::MouseEvent::Hold(x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                r#type: MouseType::Hold,
            },
            event::MouseEvent::Release(x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                r#type: MouseType::Release,
            },
            event::MouseEvent::Press(r#type, x, y) => Self {
                x: x as u32 - 1,
                y: y as u32 - 1,
                r#type: r#type.into(),
            },
        }
    }
}

/// what kind of mouse event it is
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum MouseType {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
    /// mouse release
    Release,
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
