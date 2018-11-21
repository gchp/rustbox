extern crate rustbox;

use std::time::Duration;
use std::thread::sleep;


use rustbox::RustBox;
use rustbox::{Style, Color};


fn main() {
    let mut rustbox = RustBox::new();
    rustbox.print_char(0, 0, Style::Normal, Color::White, Color::Black, 'y');
    rustbox.print_char(1, 0, Style::Underline, Color::White, Color::Black, 'y');
    rustbox.print_char(2, 0, Style::Bold, Color::White, Color::Black, 'y');
    rustbox.print_char(3, 0, Style::Blink, Color::White, Color::Black, 'y');
    rustbox.print_char(4, 0, Style::Reverse, Color::White, Color::Black, 'y');
    rustbox.print_char(1, 1, Style::Normal, Color::Red, Color::Black, 'y');
    rustbox.print_char(2, 1, Style::Normal, Color::Black, Color::Red, 'y');
    rustbox.present();

    sleep(Duration::new(2, 0));
}
