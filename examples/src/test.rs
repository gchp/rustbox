extern crate rustbox;

use std::char;

fn main() {
    rustbox::init();
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
