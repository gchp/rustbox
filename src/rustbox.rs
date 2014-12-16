extern crate libc;
extern crate "termbox-sys" as termbox;

pub use self::style::{Style, TB_BOLD, TB_UNDERLINE, TB_REVERSE};

use std::sync::atomic::{mod, AtomicBool};
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

const TB_DEFAULT: u16 = 0x00;
const TB_BLACK: u16 = 0x01;
const TB_RED: u16 = 0x02;
const TB_GREEN: u16 = 0x03;
const TB_YELLOW: u16 = 0x04;
const TB_BLUE: u16 = 0x05;
const TB_MAGENTA: u16 = 0x06;
const TB_CYAN: u16 = 0x07;
const TB_WHITE: u16 = 0x08;

mod style {
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

    impl Style {
        pub fn from_color(color: super::Color) -> Style {
            Style { bits: color as u16 & TB_NORMAL_COLOR.bits }
        }
    }
}

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0 };

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
pub enum InitErrorKind {
    AlreadyOpen = 0,
    UnsupportedTerminal = -1,
    FailedToOpenTty = -2,
    PipeTrapError = -3,
}

pub type InitError = Option<InitErrorKind>;

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            Some(InitErrorKind::AlreadyOpen) => "Termbox is already open.",
            Some(InitErrorKind::UnsupportedTerminal) => "Unsupported terminal.",
            Some(InitErrorKind::FailedToOpenTty) => "Failed to open TTY.",
            Some(InitErrorKind::PipeTrapError) =>
                "Pipe trap error.  \
                 Termbox uses unix pipes in order to deliver a message from a signal handler to \
                 the main event reading loop.",
            None => "Unknown error."
        }
    }
}

// The state of the RustBox is protected by the lock.  Yay, global state!
static RUSTBOX_RUNNING: AtomicBool = atomic::INIT_ATOMIC_BOOL;

/// true iff RustBox is currently running.  Beware of races here--don't rely on this for anything
/// critical unless you happen to know that RustBox cannot change state when it is called (a good
/// usecase would be checking to see if it's worth risking double printing backtraces to avoid
/// having them swallowed up by RustBox).
pub fn running() -> bool {
    RUSTBOX_RUNNING.load(atomic::SeqCst)
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    no_sync: marker::NoSync, // Termbox is not thread safe
}

impl RustBox {
    pub fn new() -> Result<RustBox, InitError> {
        // Ensure that we are not already running and simultaneously set RUSTBOX_RUNNING using an
        // atomic swap.  This ensures that contending threads don't trample each other.
        if RUSTBOX_RUNNING.swap(true, atomic::SeqCst) {
            // The Rustbox was already running.
            Err(Some(InitErrorKind::AlreadyOpen))
        } else {
            // The Rustbox was not already running.
            unsafe {
                match termbox::tb_init() {
                    0 => Ok(RustBox { no_sync: marker::NoSync }),
                    res => {
                        // Remember to unset RUSTBOX_RUNNING.
                        // Probably don't need SeqCst, but as noted elsewhere, better safe than
                        // sorry.
                        RUSTBOX_RUNNING.store(false, atomic::SeqCst);
                        Err(FromPrimitive::from_int(res as int))
                    }
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
        let fg = Style::from_color(fg) | (sty & self::style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        for (i, ch) in s.as_slice().chars().enumerate() {
            unsafe {
                self.change_cell(x+i, y, ch as u32, fg.bits(), bg.bits());
            }
        }
    }

    pub fn print_char(&self, x: uint, y: uint, sty: Style, fg: Color, bg: Color, ch: char) {
        let fg = Style::from_color(fg) | (sty & self::style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        unsafe {
            self.change_cell(x, y, ch as u32, fg.bits(), bg.bits());
        }
    }

    pub fn poll_event(&self) -> EventResult<Event> {
        unsafe {
            let ev = NIL_RAW_EVENT;
            let rc = termbox::tb_poll_event(&ev as *const RawEvent);
            unpack_event(rc, &ev)
        }
    }

    /// Returns Err(()) on EOF
    pub fn peek_event(&self, timeout: uint) -> EventResult<Event> {
        unsafe {
            let ev = NIL_RAW_EVENT;
            let rc = termbox::tb_peek_event(&ev as *const RawEvent, timeout as c_uint);
            unpack_event(rc, &ev)
        }
    }
}

impl Drop for RustBox {
    fn drop(&mut self) {
        // Since only one instance of the RustBox is ever accessible, we should not
        // need to do this atomically.
        // Note: we should definitely have RUSTBOX_RUNNING = true here.
        unsafe {
            termbox::tb_shutdown();
        }
        // Indicate that we're free now.  We could probably get away with lower atomicity here,
        // but there's no reason to take that chance.
        RUSTBOX_RUNNING.store(false, atomic::SeqCst);
    }
}
