#![feature(io)]

extern crate rustbox;

use std::char;
use std::old_io::stdio;
use std::error::Error;

use rustbox::{Color, RustBox, InitOption};

fn main() {
    let options = [
        if stdio::stderr_raw().isatty() { Some(InitOption::BufferStderr) } else { None },
    ];
    let rustbox = match RustBox::init(&options) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event() {
            Ok(rustbox::Event::KeyEvent(_, _, ch)) => {
                match char::from_u32(ch) {
                    Some('q') => { break; },
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
