extern crate rustbox;

use std::thread::sleep;
use std::time::Duration;

use rustbox::RustBox;
use rustbox::{Color, Event, Key, Style};

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

    rustbox.poll_event();

    // loop {
    //     match rustbox.poll_event() {
    //         Ok(Event::Key(Key::Esc)) => break,
    //         Ok(Event::Key(Key::Char(c))) => {
    //             rustbox.print_char(10, 10, Style::Normal, Color::White, Color::Black, c);
    //             rustbox.present();
    //         }
    //         _ => break,
    //     }
    // }

    // sleep(Duration::new(2, 0));
}
