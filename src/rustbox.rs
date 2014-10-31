#![feature(link_args)]
#![feature(struct_variant)]

extern crate libc;
use libc::types::os::arch::c95::{c_int, c_uint};

pub struct RawEvent {
    etype: u8,
    emod: u8,
    key: u16,
    ch: u32,
    w: i32,
    h: i32,
}

pub enum Event {
    KeyEvent(u8, u16, u32),
    ResizeEvent(i32, i32),
    NoEvent
}

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

pub enum Style {
    Normal,
    Bold,
    Underline,
    BoldUnderline
}

mod termbox {
    use libc::types::os::arch::c95::{c_int, c_uint};

    #[link(name="termbox")]
    extern {
        pub fn tb_init() -> c_int;
        pub fn tb_shutdown();

        pub fn tb_width() -> c_uint;
        pub fn tb_height() -> c_uint;

        pub fn tb_clear();
        pub fn tb_present();

        pub fn tb_set_cursor(cx: c_int, cy: c_int);
        pub fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

        //pub fn tb_select_input_mode(mode: c_int) -> c_int;
        //pub fn tb_set_clear_attributes(fg: u16, bg: u16);

        pub fn tb_peek_event(ev: *const ::RawEvent, timeout: c_uint) -> c_int;
        pub fn tb_poll_event(ev: *const ::RawEvent) -> c_int;
    }
}

fn nil_raw_event() -> RawEvent {
    RawEvent{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

fn unpack_event(ev_type: c_int, ev: &RawEvent) -> Event {
    match ev_type {
        0 => NoEvent,
        1 => {
            return KeyEvent(ev.emod, ev.key, ev.ch);
        },
        2 => {
            return ResizeEvent(ev.w, ev.h);
        },
        _ => { panic!("Unknown event"); }
    }
}

pub fn convert_color(c: Color) -> u16 {
    match c {
        Black   => 0x00,
        Red     => 0x01,
        Green   => 0x02,
        Yellow  => 0x03,
        Blue    => 0x04,
        Magenta => 0x05,
        Cyan    => 0x06,
        White   => 0x07,
    }
}

pub fn convert_style(sty: Style) -> u16 {
    match sty {
        Normal         => 0x00,
        Bold           => 0x10,
        Underline      => 0x20,
        BoldUnderline => 0x30,
    }
}

pub fn init() -> int {
    unsafe {
        termbox::tb_init() as int
    }
}

pub fn shutdown() {
    unsafe { termbox::tb_shutdown() }
}

pub fn width() -> uint {
    unsafe { termbox::tb_width() as uint }
}

pub fn height() -> uint {
    unsafe { termbox::tb_height() as uint }
}

pub fn clear() {
    unsafe { termbox::tb_clear() }
}

pub fn present() {
    unsafe { termbox::tb_present() }
}

pub fn set_cursor(x: int, y: int) {
    unsafe { termbox::tb_set_cursor(x as c_int, y as c_int) }
}

pub fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) {
    unsafe {
        termbox::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg)
    }
}

pub fn print(x: uint, y: uint, sty: Style, fg: Color, bg: Color, s: String) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for (i, ch) in s.as_slice().chars().enumerate() {
        change_cell(x+i, y, ch as u32, fg, bg);
    }
}

pub fn print_char(x: uint, y: uint, sty: Style, fg: Color, bg: Color, ch: char) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    change_cell(x, y, ch as u32, fg, bg);
}

pub fn poll_event() -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = termbox::tb_poll_event(&ev as *const RawEvent);
        return unpack_event(rc, &ev);
    }
}

pub fn peek_event(timeout: uint) -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = termbox::tb_peek_event(&ev as *const RawEvent, timeout as c_uint);
        return unpack_event(rc, &ev);
    }
}
