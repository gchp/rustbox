#![feature(io)]
#![feature(core)]

extern crate rustbox;

use std::char;
use std::old_io::stdio;
use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox, InitOptions};

fn main() {
    let rustbox = match RustBox::init(InitOptions {
        buffer_stderr: stdio::stderr_raw().isatty(),
        ..Default::default()
    }) {
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
