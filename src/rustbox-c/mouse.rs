#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Release,
    WheelUp,
    WheelDown
}

impl Mouse {
    pub fn from_code(code: u16) -> Option<Mouse> {
        match code {
            65513 => Some(Mouse::Left),
            65512 => Some(Mouse::Right),
            65511 => Some(Mouse::Middle),
            65510 => Some(Mouse::Release),
            65509 => Some(Mouse::WheelUp),
            65508 => Some(Mouse::WheelDown),
            _ => None
        }
    }
}
