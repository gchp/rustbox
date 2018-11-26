extern crate libc;

use std::mem;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::convert::From;

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
    build_term_code!(EnterCa, "?1049h\x1b[22;0;0t");
    build_term_code!(ExitCa, "?1049l\x1b[23;0;0t");
    build_term_code!(ClearScreen, "H\x1b[2J");
    build_term_code!(HideCursor, "?25l");
    build_term_code!(ShowCursor, "?25h");
    build_term_code!(SGR0, "m\x0f");

    pub struct EnterKeypad;
    impl std::fmt::Display for EnterKeypad {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "\x1B=")
        }
    }
    pub struct ExitKeypad;
    impl std::fmt::Display for ExitKeypad {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "\x1B>")
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Up,
    Down,
    Delete,
    Insert,

    Home,
    End,
    PageUp,
    PageDown,

    Char(char),
    Ctrl(char),
    F(u32),
    Unknown(u16),
}
impl Key {
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
            28 => Some(Key::Ctrl('\\')),
            29 => Some(Key::Ctrl(']')),
            30 => Some(Key::Ctrl('6')),
            31 => Some(Key::Ctrl('/')),
            32 => Some(Key::Char(' ')),
            127 => Some(Key::Backspace),
            65514 => Some(Key::Right),
            65515 => Some(Key::Left),
            65516 => Some(Key::Down),
            65517 => Some(Key::Up),
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
            65523 => Some(Key::Insert),
            65522 => Some(Key::Delete),
            65521 => Some(Key::Home),
            65520 => Some(Key::End),
            65519 => Some(Key::PageUp),
            65518 => Some(Key::PageDown),
            _     => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Key(Key),
}

/// Buffered file writing
///
/// Mostly adapted from std::io::BufWriter
struct BufferedFile {
    inner: File,
    buf: Vec<u8>,
}

impl BufferedFile {
    pub fn new(f: File) -> BufferedFile {
        BufferedFile {
            inner: f,
            buf: Vec::new(),
        }
    }

    fn flush_buffer(&mut self) -> std::io::Result<()> {
        let mut written = 0;
        let len = self.buf.len();
        let mut ret = Ok(());

        while written < len {
            match self.inner.write(&self.buf[written..]) {
                Ok(0) => {
                    ret = Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "failed to write data"));
                    break;
                }
                Ok(n) => written += n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => { ret = Err(e); break }
            }
        }
        if written > 0 {
            self.buf.drain(..written);
        }

        ret
    }
}

impl Write for BufferedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(&mut self.buf, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_buffer().and_then(|()| self.inner.flush())
    }
}


impl Drop for BufferedFile {
    fn drop(&mut self) {
        let _ = self.flush();
    }
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
    Underline,
    Bold,
    Blink,
    Reverse,
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
            Color::Black => 0,
            Color::Red => 1,
            Color::White => 7,
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
    outf: BufferedFile,
    inf: File,

    input_buffer: String,

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

        let outf = OpenOptions::new().write(true).open("/dev/tty").unwrap();
        let inf = OpenOptions::new().read(true).open("/dev/tty").unwrap();
        // TODO(gchp): find out what this is about. See termbox tb_init.
        unsafe { libc::tcsetattr(outf.as_raw_fd(), libc::TCSAFLUSH, &ios); }

        let win_size = libc::winsize { ws_col: 0, ws_row: 0, ws_xpixel: 0, ws_ypixel: 0};
        unsafe { libc::ioctl(outf.as_raw_fd(), libc::TIOCGWINSZ, &win_size); }

        let mut buffered_file = BufferedFile::new(outf);

        set_terminal_attr(&ios);


        write!(buffered_file, "{}", termcodes::EnterCa);
        write!(buffered_file, "{}", termcodes::EnterKeypad);
        write!(buffered_file, "{}", termcodes::HideCursor);
        write!(buffered_file, "{}", termcodes::SGR0);
        write!(buffered_file, "{}", termcodes::ClearScreen);

        let _ = buffered_file.flush();



        let mut back_buffer = Vec::new();
        for _i in 0..win_size.ws_row {
            let mut row = Vec::new();
            for _j in 0..win_size.ws_col {
                row.push(Cell { ch: ' ', fg: Color::White, bg: Color::Black, style: Style::Normal })
            }
            back_buffer.push(row);
        }

        RustBox {
            orig_ios: orig_ios,

            outf: buffered_file,
            inf: inf,

            input_buffer: String::new(),

            front_buffer: back_buffer.clone(),
            back_buffer: back_buffer,
            width: win_size.ws_col,
            height: win_size.ws_row,
        }
    }

    pub fn print_char(&mut self, x: usize, y: usize, style: Style, fg: Color, bg: Color, ch: char) {
        let cell = &mut self.back_buffer[y][x];

        cell.ch = ch;
        cell.bg = bg;
        cell.fg = fg;
        cell.style = style;
    }

    pub fn present(&mut self) {
        // TODO(gchp): do we need multiple buffers here?
        self.front_buffer = self.back_buffer.clone();

        for (i, _row) in self.front_buffer.iter().enumerate() {
            for cell in &self.front_buffer[i] {
                // reset
                write!(self.outf, "{}", termcodes::SGR0);

                match cell.style {
                    Style::Normal => {}
                    Style::Underline => { write!(self.outf, "\x1b[4m"); }
                    Style::Bold => { write!(self.outf, "\x1b[1m"); }
                    Style::Blink => { write!(self.outf, "\x1b[5m"); }
                    Style::Reverse => { write!(self.outf, "\x1b[7m"); }
                }

                // TODO(gchp): this currently assumes 256 colors
                let fg = cell.fg.as_256_color() & 0xFF;
                let bg = cell.bg.as_256_color() & 0xFF;

                write!(self.outf, "\x1b[38;5;{}m", fg);
                write!(self.outf, "\x1b[48;5;{}m", bg);

                write!(self.outf, "{}", cell.ch);

                // reset fg
                // write!(self.outf, "\x1b[39m");

                // reset bg
                // write!(self.outf, "\x1b[49m");
            }
        }

        let _ = self.outf.flush();

    }
}


impl Drop for RustBox {
    fn drop(&mut self) {
        write!(self.outf, "{}", termcodes::ShowCursor);
        write!(self.outf, "{}", termcodes::ClearScreen);
        write!(self.outf, "{}", termcodes::ExitCa);
        write!(self.outf, "{}", termcodes::ExitKeypad);

        set_terminal_attr(&self.orig_ios);
    }
}


struct InputBuffer<R> {
    source: R,
    leftover: Option<u8>,
}

impl <R: Read> InputBuffer<R> {
    fn new(source: R) -> InputBuffer<R> {
        InputBuffer {
            source: source,
            leftover: None,
        }
    }
}

impl <R: Read> std::iter::Iterator for InputBuffer<R> {
    type Item = Result<Event, std::io::Error>;

    fn next(&mut self) -> Option<Result<Event, std::io::Error>> {
        let mut buf = [0u8; 2];
        let mut source = &mut self.source;

        if let Some(c) = self.leftover {
            self.leftover = None;
            return Some(parse_item(c, &mut source.bytes()));
        }

        let res = match source.read(&mut buf) {
            Ok(0) => return None,
            Ok(1) => {
                match buf[0] {
                    b'\x1b' => Ok(Event::Key(Key::Esc)),
                    _ => parse_item(buf[0], &mut source.bytes()),
                }
            }
            Ok(2) => {
                let mut option_iter = &mut Some(buf[1]).into_iter();
                let result = {
                    let mut iter = option_iter.map(|c| Ok(c)).chain(source.bytes());
                    parse_item(buf[0], &mut iter)
                };
                self.leftover = option_iter.next();
                result
            }
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        };

        Some(res)
    }
}

fn parse_item<I>(c: u8, iter: &mut I) -> Result<Event, std::io::Error>
    where I: Iterator<Item = Result<u8, std::io::Error>>
{
    match c {
        b'\x1B' => {
            match iter.next() {
                Some(Ok(b'[')) => {
                    match iter.next() {
                        Some(Ok(b'D')) => Ok(Event::Key(Key::Left)),
                        Some(Ok(b'C')) => Ok(Event::Key(Key::Right)),
                        Some(Ok(b'A')) => Ok(Event::Key(Key::Up)),
                        Some(Ok(b'B')) => Ok(Event::Key(Key::Down)),
                        Some(Ok(b'H')) => Ok(Event::Key(Key::Home)),
                        Some(Ok(b'F')) => Ok(Event::Key(Key::End)),
                        _ => unimplemented!(),
                    }
                }

                c => unimplemented!("{:?}", c),
            }

        }
        c if c.is_ascii_alphanumeric() => Ok(Event::Key(Key::Char(c as char))),
        _ => { unimplemented!() }
    }
}



#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    macro_rules! key_test {
        ($name:ident, $input:expr, $($keys:expr),*) => {
            #[test]
            fn $name() {
                let source = Cursor::new(String::from($input));
                let mut buffer = InputBuffer::new(source);

                $(assert_eq!(buffer.next().unwrap().unwrap(), $keys);)*
                assert!(buffer.next().is_none());
            }
        }
    }

    key_test!(
        test_input_double_key, "ab",
        Event::Key(Key::Char('a')),
        Event::Key(Key::Char('b'))
    );
    key_test!(
        test_input_triple_key, "abc",
        Event::Key(Key::Char('a')),
        Event::Key(Key::Char('b')),
        Event::Key(Key::Char('c'))
    );
    key_test!(test_input_left_arrow_key, "\x1B[D", Event::Key(Key::Left));
    key_test!(test_input_right_arrow_key, "\x1B[C", Event::Key(Key::Right));
    key_test!(test_input_up_arrow_key, "\x1B[A", Event::Key(Key::Up));
    key_test!(test_input_down_arrow_key, "\x1B[B", Event::Key(Key::Down));
    key_test!(test_input_home_key, "\x1B[H", Event::Key(Key::Home));
    key_test!(test_input_end_arrow_key, "\x1B[F", Event::Key(Key::End));
    key_test!(test_input_esc_key, "\x1B", Event::Key(Key::Esc));
    key_test!(test_input_single_key, "a", Event::Key(Key::Char('a')));

}
