# Rustbox

Rustbox is a Rust implementation of [termbox](http://github.com/nsf/termbox).

Currently, this is just a wrapper of the C library by nsf, though my plan is to convert it to be a pure Rust implementation and remove the requirement on the C library.

The original implementation of this was inspired by [Aaron Pribadi](http://github.com/apribadi/rust-termbox), so big props to him for the original work.

**NOTE** This is under development, and the APIs may change as I figure out more how Rust works and as the language itself changes

## Usage

The best way to use Rustbox is in your Cargo config file. You are using [Cargo](http://github.com/rust-lang/cargo), right? ;)

In your `Cargo.toml`:

```
[package]
name = "example"
version = "0.1.0"
authors = ["You <you@example.com>"]

[dependencies.rustbox]
git = "https://github.com/gchp/rustbox.git"

[[bin]]
name = "example"
```

Then, in your  `src/example.rs`:

```rust
extern crate rustbox;

fn main() {
    rustbox::init();
    rustbox::print(1, 1, rustbox::Bold, rustbox::White, rustbox::Black, "Hello, world!");
    rustbox::present();

    std::io::timer::sleep(1000);

    rustbox::shutdown();
}
```
