use termion::event::Key as TermionKey;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    code: KeyCode,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyCode {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Null byte.
    Null,
    /// Esc key.
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyModifier {
    Alt,
    Ctrl,
    None,
}
