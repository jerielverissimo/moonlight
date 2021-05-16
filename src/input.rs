use termion::input::TermRead;

use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
    Key(Key),                 // termion type
    MouseButton(MouseButton), // termion type
    WindowSize { width: u16, height: u16 },
}

// Mapping types from termion and not exposes termion crate

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
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
}

/// A key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
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
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
    Unsupported,
}

impl From<termion::event::Key> for Key {
    fn from(key: termion::event::Key) -> Self {
        match key {
            termion::event::Key::Backspace => Self::Backspace,
            termion::event::Key::Left => Self::Left,
            termion::event::Key::Right => Self::Right,
            termion::event::Key::Up => Self::Up,
            termion::event::Key::Down => Self::Down,
            termion::event::Key::Home => Self::Home,
            termion::event::Key::End => Self::End,
            termion::event::Key::PageUp => Self::PageUp,
            termion::event::Key::PageDown => Self::PageDown,
            termion::event::Key::BackTab => Self::BackTab,
            termion::event::Key::Delete => Self::Delete,
            termion::event::Key::Insert => Self::Insert,
            termion::event::Key::F(f) => Self::F(f),
            termion::event::Key::Char(c) => Self::Char(c),
            termion::event::Key::Alt(c) => Self::Alt(c),
            termion::event::Key::Ctrl(c) => Self::Ctrl(c),
            termion::event::Key::Null => Self::Null,
            termion::event::Key::Esc => Self::Esc,
            termion::event::Key::__IsNotComplete => Self::Unsupported,
        }
    }
}

pub(crate) fn receive_inputs<MSG, I>(input: I, mut input_sender: crate::ChannelSender<MSG>)
where
    I: Fn(InputEvent) -> Option<MSG> + Send + 'static,
{
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            event => {
                if let Some(msg) = input(InputEvent::Key(event.into())) {
                    input_sender.send(msg);
                }
            }
        }
    }
}
