extern crate rustbox;

use std::char;
use std::error::Error;

use rustbox::{Color, EventErrorKind, RustBox};

fn main() {
    let rustbox = RustBox::init().unwrap();
    rustbox.print(1, 1, rustbox::TB_BOLD, Color::White, Color::Black, "Hello, world!".to_string());
    rustbox.print(1, 3, rustbox::TB_BOLD, Color::White, Color::Black, "Press 'q' to quit.".to_string());
    rustbox.present();

    loop {
        match rustbox.poll_event() {
            Ok(rustbox::Event::KeyEvent(_, _, ch)) => {
                match char::from_u32(ch) {
                    Some('q') => { break; },
                    _ => {}
                }
            },
            Err(Some(EventErrorKind::EOF)) => break,
            Err(e) => panic!("Unexpected error: {}", e.description()),
            _ => { }
        }
    }
}
