# Rustbox

Rustbox is a Rust implementation of [termbox](http://github.com/nsf/termbox).

Currently, this is just a wrapper of the C library by nsf, though my plan is to convert it to be a pure Rust implementation and remove the requirement on the C library.

The original implementation of this was inspired by [Aaron Pribadi](http://github.com/apribadi/rust-termbox), so big props to him for the original work.

**NOTE** This is under development, and the APIs may change as I figure out more how Rust works and as the language itself changes


## Usage

In your `Cargo.toml` add the following:

```toml
[dependencies]
rustbox = "*"
```

You can also use the current git version by instead adding:

```toml
[dependencies.rustbox]
git = "https://github.com/gchp/rustbox.git"
```

Then, in your `src/example.rs`:

```rust
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
```

**NOTE:** this example can also be run with `cargo run --example hello-world`.

### Projects that use this crate:

* [hostblock](https://github.com/cgag/hostblock)
* [rust-2048](https://github.com/kdar/rust-2048)
* [marching-squares](https://github.com/crespyl/marching-squares)
