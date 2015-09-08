#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// This enumeration represents the different keys on the keyboard. There are separate names for each special key, and for plain chars and modifiers, there are tuple names.
pub enum Key {
    /// Tab key
    Tab,
    /// Return or enter key
    Enter,
    /// Escape key
    Esc,
    /// Backspace key
    Backspace,
    /// Right arrow
    Right,
    /// Left arrow
    Left,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Delete key (this is the Mac equivilant of the backspace key)
    Delete,

    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,

    /// Specific character on keyboard, representable by single Rust char
    Char(char),
    /// Character on the keyboard, pressed simultaniusly with the Control key
    Ctrl(char),
    /// A function key, specified by a `u32`
    F(u32),
}

impl Key {
    /// Converts a char code to an optional `Key`
    pub fn from_code(code: u16) -> Option<Key> {
        match code {
            1 => Some(Key::Ctrl('a')),
            2 => Some(Key::Ctrl('b')),
            3 => Some(Key::Ctrl('c')),
            4 => Some(Key::Ctrl('d')),
            5 => Some(Key::Ctrl('e')),
            6 => Some(Key::Ctrl('f')),
            7 => Some(Key::Ctrl('g')),
            8 => Some(Key::Ctrl('h')),
            9 => Some(Key::Tab),
            10 => Some(Key::Ctrl('j')),
            11 => Some(Key::Ctrl('k')),
            12 => Some(Key::Ctrl('l')),
            13 => Some(Key::Enter),
            14 => Some(Key::Ctrl('n')),
            15 => Some(Key::Ctrl('o')),
            16 => Some(Key::Ctrl('p')),
            17 => Some(Key::Ctrl('q')),
            18 => Some(Key::Ctrl('r')),
            19 => Some(Key::Ctrl('s')),
            20 => Some(Key::Ctrl('t')),
            21 => Some(Key::Ctrl('u')),
            22 => Some(Key::Ctrl('v')),
            23 => Some(Key::Ctrl('w')),
            24 => Some(Key::Ctrl('x')),
            25 => Some(Key::Ctrl('y')),
            26 => Some(Key::Ctrl('z')),
            27 => Some(Key::Esc),
            31 => Some(Key::Ctrl('/')),
            32 => Some(Key::Char(' ')),
            127 => Some(Key::Backspace),
            65514 => Some(Key::Right),
            65515 => Some(Key::Left),
            65516 => Some(Key::Down),
            65517 => Some(Key::Up),
            65522 => Some(Key::Delete),
            65535 => Some(Key::F(1)),
            65534 => Some(Key::F(2)),
            65533 => Some(Key::F(3)),
            65532 => Some(Key::F(4)),
            65531 => Some(Key::F(5)),
            65530 => Some(Key::F(6)),
            65529 => Some(Key::F(7)),
            65528 => Some(Key::F(8)),
            65527 => Some(Key::F(9)),
            65526 => Some(Key::F(10)),
            65525 => Some(Key::F(11)),
            65524 => Some(Key::F(12)),
            65521 => Some(Key::Home),
            65520 => Some(Key::End),
            65519 => Some(Key::PageUp),
            65518 => Some(Key::PageDown),
            _     => None,
        }
    }
}
