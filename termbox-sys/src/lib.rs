extern crate libc;

use libc::{c_int, c_uint};

#[repr(C)]
#[deriving(Copy)]
pub struct RawEvent {
    pub etype: u8,
    pub emod: u8,
    pub key: u16,
    pub ch: u32,
    pub w: i32,
    pub h: i32,
}

extern "C" {
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
