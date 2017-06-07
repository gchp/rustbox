extern crate rustbox;

use std::default::Default;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use rustbox::{Color, RustBox};
use rustbox::Key;

fn spawn_render_thread(rustbox:Arc<Mutex<RustBox>>) {
    let started = Instant::now();
    thread::spawn(move || {
        {
            let mut rustbox = rustbox.lock().unwrap();
            rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black,
                          &format!("Elapsed: {:?}", started.elapsed()));
            rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                          "Press 'q' to quit.");
            rustbox.present();
        }

        loop {
            let mut rustbox = rustbox.lock().unwrap();
            rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black,
                          &format!("Elapsed: {:?}", started.elapsed()));
            rustbox.present();
        }
    });
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => Arc::new(Mutex::new(v)),
        Result::Err(e) => panic!("{}", e),
    };

    spawn_render_thread(rustbox.clone());

    loop {
        let mut rustbox = rustbox.lock().unwrap();
        match rustbox.peek_event(Duration::new(0, 0), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
        thread::yield_now();
    }
}
