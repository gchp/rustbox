extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox, OutputMode, Style};
use rustbox::Key;

fn main() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    rustbox.set_output_mode(OutputMode::EightBit);

    rustbox.print(1, 1, Style::RB_BOLD, Color::Byte(0xa2), Color::Black, "Hello, world!");
    rustbox.print(1, 3, Style::RB_NORMAL, Color::Black, Color::Byte(0x9a), "Press 'q' to quit.");
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
