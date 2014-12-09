# Rustbox

Rustbox is a Rust implementation of [termbox](http://github.com/nsf/termbox).

Currently, this is just a wrapper of the C library by nsf, though my plan is to convert it to be a pure Rust implementation and remove the requirement on the C library.

The original implementation of this was inspired by [Aaron Pribadi](http://github.com/apribadi/rust-termbox), so big props to him for the original work.

**NOTE** This is under development, and the APIs may change as I figure out more how Rust works and as the language itself changes

## Usage

Before you can use `rustbox` you'll need to install the termbox C library. To do this you can simply run the `install_termbox.sh` script
provided in this repository.

The best way to use Rustbox is in your Cargo config file. You are using [Cargo](http://github.com/rust-lang/cargo), right? ;)

In your `Cargo.toml` add the following:

```
[dependencies]
rustbox = "0.1.0"
```

You can also use the current git version by instead adding:

```
[dependencies.rustbox]
git = "https://github.com/gchp/rustbox.git"
```

Then, in your  `src/example.rs`:

```rust
extern crate rustbox;

use std::char;
use rustbox::{Style,Color};

fn main() {
    rustbox::init();
    rustbox::print(1, 1, Style::Bold, Color::White, Color::Black, "Hello, world!".to_string());
    rustbox::print(1, 3, Style::Bold, Color::White, Color::Black, "Press 'q' to quit.".to_string());
    rustbox::present();

    loop {
        match rustbox::poll_event() {
            rustbox::Event::KeyEvent(_, _, ch) => {
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
```
