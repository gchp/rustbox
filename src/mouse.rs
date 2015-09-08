#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// This enumeration represents the type of event that has been recieved from the mouse.
pub enum Mouse {
    /// The user has pressed the left mouse button
    Left,
    /// The user has pressed the right mouse button
    Right,
    /// The user has pressed the middle mouse button
    Middle,
    /// The user has released a mouse button
    Release,
    /// The user has scrolled up
    WheelUp,
    /// The user has scrolled down
    WheelDown
}

impl Mouse {
    /// Converts a mouse code to an optional `Mouse`
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
