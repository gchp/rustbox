extern crate rustbox;

use std::char;

fn main() {
    rustbox::init();
    rustbox::print(1, 1, rustbox::Bold, rustbox::White, rustbox::Black, "Hello, world!".to_string());
    rustbox::print(1, 3, rustbox::Bold, rustbox::White, rustbox::Black, "Press 'q' to quit.".to_string());
    rustbox::present();

    loop {
        match rustbox::poll_event() {
            rustbox::KeyEvent(_, _, ch) => {
                match char::from_u32(ch) {
                    Some('q') => { break; },
                    _ => {}
                }
            },
            _ => { }
        }
    }
    rustbox::shutdown();
}
