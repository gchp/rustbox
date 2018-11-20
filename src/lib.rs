extern crate libc;

use std::mem;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::os::unix::io::AsRawFd;

use libc::termios;
use libc::c_int;

macro_rules! build_term_code {
    ($name:ident, $code:expr) => {
        pub struct $name;

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, concat!("\x1B[", $code))
            }
        }

    }
}

mod termcodes {
    build_term_code!(EnterCa, "?1049h");
    build_term_code!(ExitCa, "?1049l");
    build_term_code!(ClearScreen, "2J");
    build_term_code!(HideCursor, "?25l");
    build_term_code!(ShowCursor, "?25h");
}


pub fn get_terminal_attr() -> termios {
    extern "C" {
        pub fn tcgetattr(fd: c_int, termptr: *const termios) -> c_int;
    }
    unsafe {
        let mut ios = mem::zeroed();
        tcgetattr(0, &mut ios);
        ios
    }
}


pub fn set_terminal_attr(t: &termios) -> i32 {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const termios) -> c_int;
    }
    unsafe { tcsetattr(0, 0, t) }
}

#[derive(Copy, Clone)]
pub enum Style {
    Normal,
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
    White,
}

impl Color {
    pub fn as_256_color(&self) -> u16 {
        match self {
            Color::Black => 0x00,
            Color::Red => 0x01,
            Color::White => 0x07,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    ch: char,
    bg: Color,
    fg: Color,
    style: Style,
}



pub struct RustBox {
    orig_ios: termios,
    outf: File,

    // TODO(gchp): do we need two buffers?
    front_buffer: Vec<Vec<Cell>>,
    back_buffer: Vec<Vec<Cell>>,

    width: u16,
    height: u16,
}

impl RustBox {
    pub fn new() -> RustBox {
        let orig_ios = get_terminal_attr();
        let mut ios = get_terminal_attr();

        ios.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc:: PARMRK | libc::ISTRIP
                         | libc::INLCR | libc::IGNCR | libc::ICRNL | libc::IXON);
        ios.c_oflag &= !libc::OPOST;
        ios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
        ios.c_cflag &= !(libc::CSIZE | libc::PARENB);
        ios.c_cflag |= libc::CS8;
        ios.c_cc[libc::VMIN] = 0;
        ios.c_cc[libc::VTIME] = 0;

        set_terminal_attr(&ios);

        let mut outf = OpenOptions::new().read(true).write(true).open("/dev/tty").unwrap();

        // TODO(gchp): find out what this is about. See termbox tb_init.
        unsafe { libc::tcsetattr(outf.as_raw_fd(), libc::TCSAFLUSH, &ios); }

        write!(outf, "{}", termcodes::EnterCa);
        write!(outf, "{}", termcodes::HideCursor);
        write!(outf, "{}", termcodes::ClearScreen);

        let win_size = libc::winsize { ws_col: 0, ws_row: 0, ws_xpixel: 0, ws_ypixel: 0};
        unsafe { libc::ioctl(outf.as_raw_fd(), libc::TIOCGWINSZ, &win_size); }

        let mut back_buffer = Vec::new();
        for i in 0..win_size.ws_row {
            let mut row = Vec::new();
            for j in 0..win_size.ws_col {
                row.push(Cell { ch: 'x', fg: Color::Red, bg: Color::Black, style: Style::Normal })
            }
            back_buffer.push(row);
        }

        RustBox {
            orig_ios: orig_ios,
            outf: outf,

            front_buffer: back_buffer.clone(),
            back_buffer: back_buffer,
            width: win_size.ws_col,
            height: win_size.ws_row,
        }
    }

    pub fn print_char(&mut self, x: usize, y: usize, style: Style, fg: Color, bg: Color, ch: char) {
        let mut cell = &mut self.back_buffer[y][x];

        cell.ch = ch;
        cell.bg = bg;
        cell.fg = fg;
        cell.style = style;
    }

    pub fn present(&mut self) {
        self.front_buffer = self.back_buffer.clone();

        // just try render the first cell
        let cell = &self.front_buffer[0][0];
        // assume 256 colors
        let fg = cell.fg.as_256_color() & 0xFF;
        let bg = cell.bg.as_256_color() & 0xFF;

        let red = "\x1B[38;5;1m";
        write!(&self.outf, "{}{}", red, cell.ch);
        write!(&self.outf, "\x1B[m");
        write!(&self.outf, "{}", cell.ch);
    }
}


impl Drop for RustBox {
    fn drop(&mut self) {
        write!(self.outf, "{}", termcodes::ShowCursor);
        write!(self.outf, "{}", termcodes::ClearScreen);
        write!(self.outf, "{}", termcodes::ExitCa);

        set_terminal_attr(&self.orig_ios);
    }
}
