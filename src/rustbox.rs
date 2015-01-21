#![feature(optin_builtin_traits)]
#![allow(unstable)]

extern crate libc;
extern crate "termbox-sys" as termbox;
#[macro_use] extern crate bitflags;

pub use self::running::running;
pub use self::style::{Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use std::error::Error;
use std::fmt;
use std::time::duration::Duration;
use std::num::FromPrimitive;

use termbox::RawEvent;
use libc::{c_int, c_uint};

#[derive(Copy)]
pub enum Event {
    KeyEvent(u8, u16, u32),
    ResizeEvent(i32, i32),
    NoEvent
}

#[derive(Copy, Show)]
pub enum InputMode {
    Current = 0x00,

    /// When ESC sequence is in the buffer and it doesn't match any known
    /// ESC sequence => ESC means TB_KEY_ESC
    Esc     = 0x01,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// sequence => ESC enables TB_MOD_ALT modifier for the next keyboard event.
    Alt     = 0x02,
}

#[derive(Copy, PartialEq)]
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

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0 };

// FIXME: Rust doesn't support this enum representation.
// #[derive(Copy,FromPrimitive,Show)]
// #[repr(C,int)]
// pub enum EventErrorKind {
//     Error = -1,
// }
// pub type EventError = Option<EventErrorKind>;
#[allow(non_snake_case)]
pub mod EventErrorKind {
    #[derive(Copy,Show)]
    pub struct Error;
}

pub type EventError = Option<EventErrorKind::Error>;

pub type EventResult<T> = Result<T, EventError>;

impl Error for EventError {
    fn description(&self) -> &str {
        match *self {
            // TODO: Check errno here
            Some(EventErrorKind::Error) => "Unknown error.",
            None => "Unexpected return code."
        }
    }
}

fn unpack_event(ev_type: c_int, ev: &RawEvent) -> EventResult<Event> {
    match ev_type {
        0 => Ok(Event::NoEvent),
        1 => Ok(Event::KeyEvent(ev.emod, ev.key, ev.ch)),
        2 => Ok(Event::ResizeEvent(ev.w, ev.h)),
        // FIXME: Rust doesn't support this error representation
        // res => FromPrimitive::from_int(res as isize),
        -1 => Err(Some(EventErrorKind::Error)),
        _ => Err(None)
    }
}

#[derive(Copy,FromPrimitive,Show)]
#[repr(C,isize)]
pub enum InitErrorKind {
    UnsupportedTerminal = -1,
    FailedToOpenTty = -2,
    PipeTrapError = -3,
}

pub enum InitError {
    Opt(InitOption, Option<Box<Error + 'static>>),
    AlreadyOpen,
    TermBox(Option<InitErrorKind>),
}

impl fmt::Show for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::Opt(InitOption::BufferStderr, _) => "Could not redirect stderr.",
            InitError::Opt(InitOption::InputMode(_), _) => "Could not set input mode.",
            InitError::AlreadyOpen => "RustBox is already open.",
            InitError::TermBox(e) => e.map_or("Unexpected TermBox return code.", |e| match e {
                InitErrorKind::UnsupportedTerminal => "Unsupported terminal.",
                InitErrorKind::FailedToOpenTty => "Failed to open TTY.",
                InitErrorKind::PipeTrapError => "Pipe trap error.",
            }),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InitError::Opt(_, Some(ref e)) => Some(&**e),
            _ => None
        }
    }
}

mod running {
    use std::sync::atomic::{self, AtomicBool};

    // The state of the RustBox is protected by the lock.  Yay, global state!
    static RUSTBOX_RUNNING: AtomicBool = atomic::ATOMIC_BOOL_INIT;

    /// true iff RustBox is currently running.  Beware of races here--don't rely on this for anything
    /// critical unless you happen to know that RustBox cannot change state when it is called (a good
    /// usecase would be checking to see if it's worth risking double printing backtraces to avoid
    /// having them swallowed up by RustBox).
    pub fn running() -> bool {
        RUSTBOX_RUNNING.load(atomic::Ordering::SeqCst)
    }

    // Internal RAII guard used to ensure we release the running lock whenever we acquire it.
    #[allow(missing_copy_implementations)]
    pub struct RunningGuard(());

    pub fn run() -> Option<RunningGuard> {
        // Ensure that we are not already running and simultaneously set RUSTBOX_RUNNING using an
        // atomic swap.  This ensures that contending threads don't trample each other.
        if RUSTBOX_RUNNING.swap(true, atomic::Ordering::SeqCst) {
            // The Rustbox was already running.
            None
        } else {
            // The RustBox was not already running, and now we have the lock.
            Some(RunningGuard(()))
        }
    }

    impl Drop for RunningGuard {
        fn drop(&mut self) {
            // Indicate that we're free now.  We could probably get away with lower atomicity here,
            // but there's no reason to take that chance.
            RUSTBOX_RUNNING.store(false, atomic::Ordering::SeqCst);
        }
    }
}

// RAII guard for input redirection
#[cfg(unix)]
mod redirect {
    use std::error::Error;

    use libc;
    use std::io::{util, IoError, PipeStream};
    use std::io::pipe::PipePair;
    use std::os::unix::AsRawFd;
    use super::{InitError, InitOption};
    use super::running::RunningGuard;

    pub struct Redirect {
        pair: PipePair,
        fd: PipeStream,
    }

    impl Drop for Redirect {
        fn drop(&mut self) {
            // We make sure that we never actually create the Redirect without also taking a
            // RunningGuard.  This means that we know that this will always be dropped immediately
            // before the RunningGuard is destroyed, and *after* a RustBox containing one is
            // destroyed.
            //
            // We rely on destructor order here: destructors are always executed top-down,  so as
            // long as this is included above the RunningGuard in the RustBox struct, we can be
            // confident that it is destroyed while we're still holding onto the lock.
            unsafe {
                let old_fd = self.pair.writer.as_raw_fd();
                let new_fd = self.fd.as_raw_fd();
                // Reopen new_fd as writer.
                // (Note that if we fail here, we can't really do anything about it, so just ignore any
                // errors).
                if libc::dup2(old_fd, new_fd) != new_fd { return }
            }
            // Copy from reader to writer.
            drop(util::copy(&mut self.pair.reader, &mut self.pair.writer));
        }
    }

    // The reason we take the RunningGuard is to make sure we don't try to redirect before the
    // TermBox is set up.  Otherwise it is possible to race with other threads trying to set up the
    // RustBox.
    fn redirect(new: PipeStream, _: &RunningGuard) -> Result<Redirect, Option<Box<Error + 'static>>> {
        // Create a pipe pair.
        let mut pair = try!(PipeStream::pair().map_err( |e| Some(Box::new(e) as Box<Error>)));
        unsafe {
            let new_fd = new.as_raw_fd();
            // Copy new_fd to dup_fd.
            let dup_fd = match libc::dup(new_fd) {
                -1 => return Err(Some(Box::new(IoError::last_error()) as Box<Error>)),
                fd => try!(PipeStream::open(fd).map_err( |e| Some(Box::new(e) as Box<Error>))),
            };
            // Make the writer nonblocking.  This means that even if the stderr pipe fills up,
            // exceptions from stack traces will not block the program.  Unfortunately, if this
            // does happen stderr outputwill be lost until RustBox exits.
            let old_fd = pair.writer.as_raw_fd();
            let res = libc::fcntl(old_fd, libc::F_SETFL, libc::O_NONBLOCK);
            if res != 0 {
                return Err(if res == -1 {
                    Some(Box::new(IoError::last_error()) as Box<Error>)
                } else { None }) // This should really never happen, but no reason to unwind here.
            }
            // Reopen new_fd as writer.
            let fd = libc::dup2(old_fd, new_fd);
            if fd == new_fd {
                // On success, the new file descriptor should be returned.  Replace the old one
                // with dup_fd, since we no longer need an explicit reference to the writer.
                // Note that it is *possible* that some other thread tried to take over stderr
                // between when we did and now, causing a race here.  RustBox won't do it, though.
                // And it's honestly not clear how to guarantee correct behavior there anyway,
                // since if the change had come a fraction of a second later we still probably
                // wouldn't want to overwite it.  In general this is a good argument for why the
                // redirect behavior is optional.
                pair.writer = dup_fd;
                Ok(Redirect {
                    pair: pair,
                    fd: new,
                })
            } else {
                Err(if fd == -1 { Some(Box::new(IoError::last_error()) as Box<Error>) } else { None })
            }
        }
    }

    pub fn redirect_stderr(stderr: &mut Option<Redirect>,
                           rg: &RunningGuard) -> Result<(), InitError> {
        match *stderr {
            Some(_) => {
                // Can only redirect once.
                Err(InitError::Opt(InitOption::BufferStderr, None))
            },
            None => {
                *stderr = Some(try!(redirect(
                            try!(PipeStream::open(libc::STDERR_FILENO)
                            .map_err( |e| InitError::Opt(InitOption::BufferStderr,
                                                         Some(Box::new(e) as Box<Error>)) )),
                            rg)
                        .map_err( |e| InitError::Opt(InitOption::BufferStderr, e))));
                Ok(())
            }
        }
    }
}

#[cfg(not(unix))]
// Not sure how we'll do this on Windows, unimplemented for now.
mod redirect {
    pub enum Redirect { }

    pub fn redirect_stderr(_: &mut Option<Redirect>,
                           _: &super::RunningGuard) -> Result<(), super::InitError> {
        Err(super::InitError::Opt(super::InitOption::BufferStderr, None))
    }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    // We only bother to redirect stderr for the moment, since it's used for panic!
    _stderr: Option<redirect::Redirect>,

    // RAII lock.
    //
    // Note that running *MUST* be the last field in the destructor, since destructors run in
    // top-down order.  Otherwise it will not properly protect the above fields.
    _running: running::RunningGuard,
}

// Termbox is not thread safe
impl !Send for RustBox {}

#[derive(Copy,Show)]
pub enum InitOption {
    /// Use this option to automatically buffer stderr while RustBox is running.  It will be
    /// written when RustBox exits.
    ///
    /// This option uses a nonblocking OS pipe to buffer stderr output.  This means that if the
    /// pipe fills up, subsequent writes will fail until RustBox exits.  If this is a concern for
    /// your program, don't use RustBox's default pipe-based redirection; instead, redirect stderr
    /// to a log file or another process that is capable of handling it better.
    BufferStderr,

    /// Use this option to initialize with a specific input mode
    ///
    /// See InputMode enum for details on the variants.
    InputMode(InputMode),
}

impl RustBox {
    pub fn init(opts: &[Option<InitOption>]) -> Result<RustBox, InitError> {
        // Acquire RAII lock.  This might seem like overkill, but it is easy to forget to release
        // it in the maze of error conditions below.
        let running = match running::run() {
            Some(r) => r,
            None => return Err(InitError::AlreadyOpen)
        };
        // Time to check our options.
        let mut stderr = None;
        for opt in opts.iter().filter_map(|&opt| opt) {
            match opt {
                InitOption::BufferStderr => try!(redirect::redirect_stderr(&mut stderr, &running)),
                InitOption::InputMode(mode) => unsafe { termbox::tb_select_input_mode(mode as c_int); },
            }
        }
        // Create the RustBox.
        Ok(unsafe {
            match termbox::tb_init() {
                0 => RustBox {
                    _stderr: stderr,
                    _running: running,
                },
                res => {
                    return Err(InitError::TermBox(FromPrimitive::from_int(res as isize)))
                }
            }
        })
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

    // Unsafe because u8 is not guaranteed to be a UTF-8 character
    pub unsafe fn change_cell(&self, x: usize, y: usize, ch: u32, fg: u16, bg: u16) {
        termbox::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg)
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

    pub fn poll_event(&self) -> EventResult<Event> {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_poll_event(&ev as *const RawEvent)
        };
        unpack_event(rc, &ev)
    }

    pub fn peek_event(&self, timeout: Duration) -> EventResult<Event> {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_peek_event(&ev as *const RawEvent, timeout.num_milliseconds() as c_uint)
        };
        unpack_event(rc, &ev)
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
