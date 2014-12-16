extern crate libc;
extern crate "termbox-sys" as termbox;

use std::error::Error;
use std::kinds::marker;

use termbox::RawEvent;
use libc::{c_int, c_uint};

#[deriving(Copy)]
pub enum Event {
    KeyEvent(u8, u16, u32),
    ResizeEvent(i32, i32),
    NoEvent
}

#[deriving(Copy)]
#[repr(C,u16)]
pub enum Color {
    Default = TB_DEFAULT as int,
    Black = TB_BLACK as int,
    Red = TB_RED as int,
    Green = TB_GREEN as int,
    Yellow = TB_YELLOW as int,
    Blue = TB_BLUE as int,
    Magenta = TB_MAGENTA as int,
    Cyan = TB_CYAN as int,
    White = TB_WHITE as int,
}

impl Color {
    pub fn to_style(&self) -> Style {
        Style { bits: *self as u16 & TB_NORMAL_COLOR.bits }
    }
}

const TB_DEFAULT: u16 = 0x00;
const TB_BLACK: u16 = 0x01;
const TB_RED: u16 = 0x02;
const TB_GREEN: u16 = 0x03;
const TB_YELLOW: u16 = 0x04;
const TB_BLUE: u16 = 0x05;
const TB_MAGENTA: u16 = 0x06;
const TB_CYAN: u16 = 0x07;
const TB_WHITE: u16 = 0x08;

bitflags! {
    #[repr(C)]
    flags Style: u16 {
        const TB_NORMAL_COLOR = 0x000F,
        const TB_BOLD = 0x0100,
        const TB_UNDERLINE = 0x0200,
        const TB_REVERSE = 0x0400,
        const TB_ATTRIB = TB_BOLD.bits | TB_UNDERLINE.bits | TB_REVERSE.bits,
    }
}

fn nil_raw_event() -> RawEvent {
    RawEvent{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

// FIXME: Rust doesn't support this enum representation.
// #[deriving(Copy,FromPrimitive,Show)]
// #[repr(C,int)]
// pub enum EventErrorKind {
//     EOF = -1,
// }
// pub type EventError = Option<EventErrorKind>;
#[allow(non_snake_case)]
pub mod EventErrorKind {
    #[deriving(Copy,Show)]
    pub struct EOF;
}

pub type EventError = Option<EventErrorKind::EOF>;

pub type EventResult<T> = Result<T, EventError>;

impl Error for EventError {
    fn description(&self) -> &str {
        match *self {
            Some(EventErrorKind::EOF) => "End of file",
            None => "Unknown error."
        }
    }
}

fn unpack_event(ev_type: c_int, ev: &RawEvent) -> EventResult<Event> {
    match ev_type {
        0 => Ok(Event::NoEvent),
        1 => Ok(Event::KeyEvent(ev.emod, ev.key, ev.ch)),
        2 => Ok(Event::ResizeEvent(ev.w, ev.h)),
        // FIXME: Rust doesn't support this error representation
        // res => FromPrimitive::from_int(res as int),
        -1 => Err(Some(EventErrorKind::EOF)),
        _ => Err(None)
    }
}

#[deriving(Copy,FromPrimitive,Show)]
#[repr(C,int)]
pub enum InitError {
    Unknown = 0,
    UnsupportedTerminal = -1,
    FailedToOpenTty = -2,
    PipeTrapError = -3,
}

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::Unknown => "Unknown error.",
            InitError::UnsupportedTerminal => "Unsupported terminal.",
            InitError::FailedToOpenTty => "Failed to open TTY.",
            InitError::PipeTrapError =>
                "Pipe trap error.  \
                 Termbox uses unix pipes in order to deliver a message from a signal handler to \
                 the main event reading loop.",
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    no_sync: marker::NoSync, // Termbox is not thread safe
}

impl RustBox {
    pub fn new() -> Result<RustBox, InitError> {
        unsafe {
            match termbox::tb_init() {
                0 => Ok(RustBox { no_sync: marker::NoSync }),
                res => match FromPrimitive::from_int(res as int) {
                    Some(e) => Err(e),
                    None => Err(InitError::Unknown)
                }
            }
        }
    }

    pub fn width(&self) -> uint {
        unsafe { termbox::tb_width() as uint }
    }

    pub fn height(&self) -> uint {
        unsafe { termbox::tb_height() as uint }
    }

    pub fn clear(&self) {
        unsafe { termbox::tb_clear() }
    }

    pub fn present(&self) {
        unsafe { termbox::tb_present() }
    }

    pub fn set_cursor(&self, x: int, y: int) {
        unsafe { termbox::tb_set_cursor(x as c_int, y as c_int) }
    }

    // Unsafe because u8 is not guaranteed to be a UTF-8 character
    pub unsafe fn change_cell(&self, x: uint, y: uint, ch: u32, fg: u16, bg: u16) {
        termbox::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg)
    }

    pub fn print(&self, x: uint, y: uint, sty: Style, fg: Color, bg: Color, s: String) {
        let fg = fg.to_style() | (sty & TB_ATTRIB);
        let bg = bg.to_style();
        for (i, ch) in s.as_slice().chars().enumerate() {
            unsafe {
                self.change_cell(x+i, y, ch as u32, fg.bits(), bg.bits());
            }
        }
    }

    pub fn print_char(&self, x: uint, y: uint, sty: Style, fg: Color, bg: Color, ch: char) {
        let fg = fg.to_style() | (sty & TB_ATTRIB);
        let bg = bg.to_style();
        unsafe {
            self.change_cell(x, y, ch as u32, fg.bits(), bg.bits());
        }
    }

    pub fn poll_event(&self) -> EventResult<Event> {
        unsafe {
            let ev = nil_raw_event();
            let rc = termbox::tb_poll_event(&ev as *const RawEvent);
            unpack_event(rc, &ev)
        }
    }

    /// Returns Err(()) on EOF
    pub fn peek_event(&self, timeout: uint) -> EventResult<Event> {
        unsafe {
            let ev = nil_raw_event();
            let rc = termbox::tb_peek_event(&ev as *const RawEvent, timeout as c_uint);
            unpack_event(rc, &ev)
        }
    }
}

impl Drop for RustBox {
    fn drop(&mut self) {
        unsafe {
            termbox::tb_shutdown();
        }
    }
}
