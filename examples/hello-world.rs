extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox, Style};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, Style::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, Style::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
