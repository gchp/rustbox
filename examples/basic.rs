extern crate rustbox;

use std::time::Duration;
use std::thread::sleep;


use rustbox::RustBox;
use rustbox::{Style, Color};


fn main() {
    let mut rustbox = RustBox::new();
    rustbox.print_char(0, 0, Style::Normal, Color::White, Color::Black, 'y');
    rustbox.present();

    sleep(Duration::new(2, 0));
}
