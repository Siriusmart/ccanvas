use serde::Serialize;
use termion::event::Key as TermionKey;

/// a single keyboard event
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct KeyEvent {
    /// the keycode represented by the characetr
    code: KeyCode,
    /// key modifiers (e.g. ctrl)
    modifier: KeyModifier,
}

impl TryFrom<TermionKey> for KeyEvent {
    fn try_from(value: TermionKey) -> Result<Self, Self::Error> {
        match value {
            TermionKey::Alt(c) => Ok(Self::new(KeyCode::Char(c), KeyModifier::Alt)),
            TermionKey::Ctrl(c) => Ok(Self::new(KeyCode::Char(c), KeyModifier::Ctrl)),
            TermionKey::__IsNotComplete => Err(crate::Error::UnsupportedKey),
            key => Ok(Self::new(
                KeyCode::try_from(key).unwrap(),
                KeyModifier::None,
            )),
        }
    }

    type Error = crate::Error;
}

impl KeyEvent {
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Self {
        Self { code, modifier }
    }
}

/// a unique key (non modifier keys)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum KeyCode {
    /// Backspace.
    #[serde(rename = "backspace")]
    Backspace,
    /// Left arrow.
    #[serde(rename = "left")]
    Left,
    /// Right arrow.
    #[serde(rename = "right")]
    Right,
    /// Up arrow.
    #[serde(rename = "up")]
    Up,
    /// Down arrow.
    #[serde(rename = "down")]
    Down,
    /// Home key.
    #[serde(rename = "home")]
    Home,
    /// End key.
    #[serde(rename = "end")]
    End,
    /// Page Up key.
    #[serde(rename = "pageup")]
    PageUp,
    /// Page Down key.
    #[serde(rename = "pagedown")]
    PageDown,
    /// Backward Tab key.
    #[serde(rename = "backtab")]
    BackTab,
    /// Delete key.
    #[serde(rename = "delete")]
    Delete,
    /// Insert key.
    #[serde(rename = "insert")]
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    #[serde(rename = "f")]
    F(u8),
    /// Normal character.
    #[serde(rename = "char")]
    Char(char),
    /// Null byte.
    #[serde(rename = "null")]
    Null,
    /// Esc key.
    #[serde(rename = "esc")]
    Esc,
}

impl TryFrom<TermionKey> for KeyCode {
    fn try_from(value: TermionKey) -> Result<Self, Self::Error> {
        match value {
            TermionKey::Up => Ok(Self::Up),
            TermionKey::End => Ok(Self::End),
            TermionKey::F(f) => Ok(Self::F(f)),
            TermionKey::Esc => Ok(Self::Esc),
            TermionKey::Left => Ok(Self::Left),
            TermionKey::Down => Ok(Self::Down),
            TermionKey::Home => Ok(Self::Home),
            TermionKey::Null => Ok(Self::Null),
            TermionKey::Right => Ok(Self::Right),
            TermionKey::PageUp => Ok(Self::PageUp),
            TermionKey::Delete => Ok(Self::Delete),
            TermionKey::Insert => Ok(Self::Insert),
            TermionKey::BackTab => Ok(Self::BackTab),
            TermionKey::PageDown => Ok(Self::PageDown),
            TermionKey::Backspace => Ok(Self::Backspace),
            TermionKey::Char(c) => Ok(Self::Char(c)),
            TermionKey::Alt(_) | TermionKey::Ctrl(_) | TermionKey::__IsNotComplete => {
                Err(crate::Error::UnsupportedKey)
            }
        }
    }

    type Error = crate::Error;
}

/// modifier keys that only exist as modifiers to the real key code
///
/// no shift, as it is not a real modifier
/// check if shift might be pressed yourself using is_upper_case
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum KeyModifier {
    #[serde(rename = "alt")]
    Alt,
    /// note that certain keys may not be modifiable with ctrl, due to limitations of terminals.
    #[serde(rename = "ctrl")]
    Ctrl,
    #[serde(rename = "none")]
    None,
}
