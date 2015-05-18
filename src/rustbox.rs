#![feature(libc)]
#![feature(optin_builtin_traits)]

extern crate gag;
extern crate libc;
extern crate num;
extern crate time;
extern crate termbox_sys as termbox;
#[macro_use] extern crate bitflags;

pub use self::style::{Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use std::error::Error;
use std::fmt;
use std::io;
use std::char;
use std::default::Default;

use num::FromPrimitive;
use termbox::RawEvent;
use libc::c_int;
use gag::Hold;
use time::Duration;

pub mod keyboard;
pub mod mouse;

pub use keyboard::Key;
pub use mouse::Mouse;

#[derive(Clone, Copy)]
pub enum Event {
    KeyEventRaw(u8, u16, u32),
    KeyEvent(Option<Key>),
    ResizeEvent(i32, i32),
    MouseEvent(Mouse, i32, i32),
    NoEvent
}

#[derive(Clone, Copy, Debug)]
pub enum InputMode {
    Current = 0x00,

    /// When ESC sequence is in the buffer and it doesn't match any known
    /// ESC sequence => ESC means TB_KEY_ESC
    Esc     = 0x01,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// sequence => ESC enables TB_MOD_ALT modifier for the next keyboard event.
    Alt     = 0x02,
    /// Same as `Esc` but enables mouse events
    EscMouse = 0x05,
    /// Same as `Alt` but enables mouse events
    AltMouse = 0x06
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C,u16)]
pub enum Color {
    Default =  0x00,
    Black =    0x01,
    Red =      0x02,
    Green =    0x03,
    Yellow =   0x04,
    Blue =     0x05,
    Magenta =  0x06,
    Cyan =     0x07,
    White =    0x08,
}

mod style {
    bitflags! {
        #[repr(C)]
        flags Style: u16 {
            const TB_NORMAL_COLOR = 0x000F,
            const RB_BOLD = 0x0100,
            const RB_UNDERLINE = 0x0200,
            const RB_REVERSE = 0x0400,
            const RB_NORMAL = 0x0000,
            const TB_ATTRIB = RB_BOLD.bits | RB_UNDERLINE.bits | RB_REVERSE.bits,
        }
    }

    impl Style {
        pub fn from_color(color: super::Color) -> Style {
            Style { bits: color as u16 & TB_NORMAL_COLOR.bits }
        }
    }
}

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0, x: 0, y: 0 };

#[derive(Debug)]
pub enum EventError {
   TermboxError,
   Unknown(isize),
}

impl fmt::Display for EventError {
   fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      write!(fmt, "{}", self.description())
   }
}

impl Error for EventError {
   fn description(&self) -> &str {
      match *self {
         EventError::TermboxError => "Error in Termbox",
         // I don't know how to format this without lifetime error.
         // EventError::Unknown(n) => &format!("There was an unknown error. Error code: {}", n),
         EventError::Unknown(_) => "Unknown error in Termbox",
      }
   }
}

impl FromPrimitive for EventError {
   fn from_i64(n: i64) -> Option<EventError> {
      match n {
         -1 => Some(EventError::TermboxError),
         n => Some(EventError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<EventError> {
      Some(EventError::Unknown(n as isize))
   }
}

pub type EventResult = Result<Event, EventError>;

/// Unpack a RawEvent to an Event
///
/// if the `raw` parameter is true, then the Event variant will be the raw
/// representation of the event.
///     for instance KeyEventRaw instead of KeyEvent
///
/// This is useful if you want to interpret the raw event data yourself, rather
/// than having rustbox translate it to its own representation.
fn unpack_event(ev_type: c_int, ev: &RawEvent, raw: bool) -> EventResult {
    match ev_type {
        0 => Ok(Event::NoEvent),
        1 => Ok(
            if raw {
                Event::KeyEventRaw(ev.emod, ev.key, ev.ch)
            } else {
                let k = match ev.key {
                    0 => char::from_u32(ev.ch).map(|c| Key::Char(c)),
                    a => Key::from_code(a),
                };
                Event::KeyEvent(k)
            }),
        2 => Ok(Event::ResizeEvent(ev.w, ev.h)),
        3 => {
            let mouse = Mouse::from_code(ev.key).unwrap_or(Mouse::Left);
            Ok(Event::MouseEvent(mouse, ev.x, ev.y))
        },
        // `unwrap` is safe here because FromPrimitive for EventError only returns `Some`.
        n => Err(FromPrimitive::from_isize(n as isize).unwrap()),
    }
}

#[derive(Debug)]
pub enum InitError {
    BufferStderrFailed(io::Error),
    AlreadyOpen,
    UnsupportedTerminal,
    FailedToOpenTTy,
    PipeTrapError,
    Unknown(isize),
}

impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::BufferStderrFailed(_) => "Could not redirect stderr",
            InitError::AlreadyOpen => "RustBox is already open",
            InitError::UnsupportedTerminal => "Unsupported terminal",
            InitError::FailedToOpenTTy => "Failed to open TTY",
            InitError::PipeTrapError => "Pipe trap error",
            InitError::Unknown(_) => "Unknown error from Termbox",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InitError::BufferStderrFailed(ref e) => Some(e),
            _ => None
        }
    }
}

impl FromPrimitive for InitError {
   fn from_i64(n: i64) -> Option<InitError> {
      match n {
         -1 => Some(InitError::UnsupportedTerminal),
         -2 => Some(InitError::FailedToOpenTTy),
         -3 => Some(InitError::PipeTrapError),
         n => Some(InitError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<InitError> {
      Some(InitError::Unknown(n as isize))
   }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
   // We only bother to redirect stderr for the moment, since it's used for panic!
   _stderr: Option<Hold>,
}

// Termbox is not thread safe
impl !Send for RustBox {}

#[derive(Clone, Copy,Debug)]
pub struct InitOptions {
    /// Use this option to initialize with a specific input mode
    ///
    /// See InputMode enum for details on the variants.
    pub input_mode: InputMode,

    /// Use this option to automatically buffer stderr while RustBox is running.  It will be
    /// written when RustBox exits.
    ///
    /// This option uses a nonblocking OS pipe to buffer stderr output.  This means that if the
    /// pipe fills up, subsequent writes will fail until RustBox exits.  If this is a concern for
    /// your program, don't use RustBox's default pipe-based redirection; instead, redirect stderr
    /// to a log file or another process that is capable of handling it better.
    pub buffer_stderr: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        InitOptions {
            input_mode: InputMode::Current,
            buffer_stderr: false,
        }
    }
}

impl RustBox {
    /// Initialize rustbox.
    ///
    /// For the default options, you can use:
    ///
    /// ```
    /// use rustbox::RustBox;
    /// use std::default::Default;
    /// let rb = RustBox::init(Default::default());
    /// ```
    ///
    /// Otherwise, you can specify:
    ///
    /// ```
    /// use rustbox::{RustBox, InitOptions};
    /// use std::default::Default;
    /// let rb = RustBox::init(InitOptions { input_mode: rustbox::InputMode::Esc, ..Default::default() });
    /// ```
    pub fn init(opts: InitOptions) -> Result<RustBox, InitError> {
        let stderr = if opts.buffer_stderr {
            Some(try!(Hold::stderr().map_err(|e| InitError::BufferStderrFailed(e))))
        } else {
            None
        };

        // Create the RustBox.
        let rb = unsafe { match termbox::tb_init() {
            0 => RustBox {
                _stderr: stderr,
            },
            res => {
                return Err(FromPrimitive::from_isize(res as isize).unwrap())
            }
        }};
        match opts.input_mode {
            InputMode::Current => (),
            _ => rb.set_input_mode(opts.input_mode),
        }
        Ok(rb)
    }

    pub fn width(&self) -> usize {
        unsafe { termbox::tb_width() as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { termbox::tb_height() as usize }
    }

    pub fn clear(&self) {
        unsafe { termbox::tb_clear() }
    }

    pub fn present(&self) {
        unsafe { termbox::tb_present() }
    }

    pub fn set_cursor(&self, x: isize, y: isize) {
        unsafe { termbox::tb_set_cursor(x as c_int, y as c_int) }
    }

    pub unsafe fn change_cell(&self, x: usize, y: usize, ch: u32, fg: u16, bg: u16) {
        termbox::tb_change_cell(x as c_int, y as c_int, ch, fg, bg)
    }

    pub fn print(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, s: &str) {
        let fg = Style::from_color(fg) | (sty & style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        for (i, ch) in s.chars().enumerate() {
            unsafe {
                self.change_cell(x+i, y, ch as u32, fg.bits(), bg.bits());
            }
        }
    }

    pub fn print_char(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, ch: char) {
        let fg = Style::from_color(fg) | (sty & style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        unsafe {
            self.change_cell(x, y, ch as u32, fg.bits(), bg.bits());
        }
    }

    pub fn poll_event(&self, raw: bool) -> EventResult {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_poll_event(&ev as *const RawEvent)
        };
        unpack_event(rc, &ev, raw)
    }

    pub fn peek_event(&self, timeout: Duration, raw: bool) -> EventResult {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_peek_event(&ev as *const RawEvent, timeout.num_milliseconds() as c_int)
        };
        unpack_event(rc, &ev, raw)
    }

    pub fn set_input_mode(&self, mode: InputMode) {
        unsafe {
            termbox::tb_select_input_mode(mode as c_int);
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
    }
}
