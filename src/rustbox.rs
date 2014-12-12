extern crate libc;
extern crate "termbox-sys" as termbox;

use termbox::RawEvent;
use libc::{c_int, c_uint};

#[deriving(Copy)]
pub enum Event {
    KeyEvent(u8, u16, u32),
    ResizeEvent(i32, i32),
    NoEvent
}

#[deriving(Copy)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

#[deriving(Copy)]
pub enum Style {
    Normal,
    Bold,
    Underline,
    BoldUnderline,
    Reverse
}

fn nil_raw_event() -> RawEvent {
    RawEvent{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

fn unpack_event(ev_type: c_int, ev: &RawEvent) -> Event {
    match ev_type {
        0 => Event::NoEvent,
        1 => Event::KeyEvent(ev.emod, ev.key, ev.ch),
        2 => Event::ResizeEvent(ev.w, ev.h),
        _ => panic!("Unknown event")
    }
}

pub fn convert_color(c: Color) -> u16 {
    match c {
        Color::Default => 0x00,
        Color::Black   => 0x01,
        Color::Red     => 0x02,
        Color::Green   => 0x03,
        Color::Yellow  => 0x04,
        Color::Blue    => 0x05,
        Color::Magenta => 0x06,
        Color::Cyan    => 0x07,
        Color::White   => 0x08,
    }
}

pub fn convert_style(sty: Style) -> u16 {
    match sty {
        Style::Normal         => 0x0000,
        Style::Bold           => 0x0100,
        Style::Underline      => 0x0200,
        Style::BoldUnderline  => 0x0300,
        Style::Reverse        => 0x0400,
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

pub fn print(x: uint, y: uint, sty: Style, fg: Color, bg: Color, s: &str) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for (i, ch) in s.chars().enumerate() {
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
        unpack_event(rc, &ev)
    }
}

pub fn peek_event(timeout: uint) -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = termbox::tb_peek_event(&ev as *const RawEvent, timeout as c_uint);
        unpack_event(rc, &ev)
    }
}
