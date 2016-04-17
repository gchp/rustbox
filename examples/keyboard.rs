extern crate rustbox;

use std::char;
use std::error::Error;
use std::default::Default;
use std::collections::HashMap;
use std::ascii::AsciiExt;

use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::InputMode;
use rustbox::Mouse;

#[derive(Clone, Copy, Debug)]
struct WidgetKey(usize, usize, &'static str);

static K_ESC: WidgetKey = WidgetKey(1, 1, "ESC");

static K_F1: WidgetKey = WidgetKey(6, 1, "F1");
static K_F2: WidgetKey = WidgetKey(9, 1, "F2");
static K_F3: WidgetKey = WidgetKey(12, 1, "F3");
static K_F4: WidgetKey = WidgetKey(15, 1, "F4");
static K_F5: WidgetKey = WidgetKey(19, 1, "F5");
static K_F6: WidgetKey = WidgetKey(22, 1, "F6");
static K_F7: WidgetKey = WidgetKey(25, 1, "F7");
static K_F8: WidgetKey = WidgetKey(28, 1, "F8");
static K_F9: WidgetKey = WidgetKey(33, 1, "F9");
static K_F10: WidgetKey = WidgetKey(36, 1, "F10");
static K_F11: WidgetKey = WidgetKey(40, 1, "F11");
static K_F12: WidgetKey = WidgetKey(44, 1, "F12");

static K_PRN: WidgetKey = WidgetKey(50, 1, "PRN");
static K_SCR: WidgetKey = WidgetKey(54, 1, "SCR");
static K_BRK: WidgetKey = WidgetKey(58, 1, "BRK");

static K_LED1: WidgetKey = WidgetKey(66, 1, "-");
static K_LED2: WidgetKey = WidgetKey(70, 1, "-");
static K_LED3: WidgetKey = WidgetKey(74, 1, "-");

static K_TILDE: WidgetKey = WidgetKey(1, 4, "`");
static K_TILDE_SHIFT: WidgetKey = WidgetKey(1, 4, "~");
static K_1: WidgetKey = WidgetKey(4, 4, "1");
static K_1_SHIFT: WidgetKey = WidgetKey(4, 4, "!");
static K_2: WidgetKey = WidgetKey(7, 4, "2");
static K_2_SHIFT: WidgetKey = WidgetKey(7, 4, "@");
static K_3: WidgetKey = WidgetKey(10, 4, "3");
static K_3_SHIFT: WidgetKey = WidgetKey(10, 4, "#");
static K_4: WidgetKey = WidgetKey(13, 4, "4");
static K_4_SHIFT: WidgetKey = WidgetKey(13, 4, "$");
static K_5: WidgetKey = WidgetKey(16, 4, "5");
static K_5_SHIFT: WidgetKey = WidgetKey(16, 4, "%");
static K_6: WidgetKey = WidgetKey(19, 4, "6");
static K_6_SHIFT: WidgetKey = WidgetKey(19, 4, "^");
static K_7: WidgetKey = WidgetKey(22, 4, "7");
static K_7_SHIFT: WidgetKey = WidgetKey(22, 4, "&");
static K_8: WidgetKey = WidgetKey(25, 4, "8");
static K_8_SHIFT: WidgetKey = WidgetKey(25, 4, "*");
static K_9: WidgetKey = WidgetKey(28, 4, "9");
static K_9_SHIFT: WidgetKey = WidgetKey(28, 4, "(");
static K_0: WidgetKey = WidgetKey(31, 4, "0");
static K_0_SHIFT: WidgetKey = WidgetKey(31, 4, ")");
static K_MINUS: WidgetKey = WidgetKey(34, 4, "-");
static K_MINUS_SHIFT: WidgetKey = WidgetKey(34, 4, "_");
static K_EQUALS: WidgetKey = WidgetKey(37, 4, "=");
static K_EQUALS_SHIFT: WidgetKey = WidgetKey(37, 4, "+");
static K_BACKSLASH: WidgetKey = WidgetKey(40, 4, "\\");
static K_BACKSLASH_SHIFT: WidgetKey = WidgetKey(40, 4, "|");

static K_BACKSPACE: WidgetKey = WidgetKey(44, 4, "←──");

static K_INS: WidgetKey = WidgetKey(50, 4, "INS");
static K_HOM: WidgetKey = WidgetKey(54, 4, "HOM");
static K_PGU: WidgetKey = WidgetKey(58, 4, "PGU");

static K_K_NUMLOCK: WidgetKey = WidgetKey(65, 4, "N");
static K_K_SLASH: WidgetKey = WidgetKey(68, 4, "/");
static K_K_STAR: WidgetKey = WidgetKey(71, 4, "*");
static K_K_MINUS: WidgetKey = WidgetKey(74, 4, "-");

static K_TAB: WidgetKey = WidgetKey(1, 6, "TAB");

static K_MIN_Q: WidgetKey = WidgetKey(6, 6, "q");
static K_Q: WidgetKey = WidgetKey(6, 6, "Q");
static K_MIN_W: WidgetKey = WidgetKey(9, 6, "w");
static K_W: WidgetKey = WidgetKey(9, 6, "W");
static K_MIN_E: WidgetKey = WidgetKey(12, 6, "e");
static K_E: WidgetKey = WidgetKey(12, 6, "E");
static K_MIN_R: WidgetKey = WidgetKey(15, 6, "r");
static K_R: WidgetKey = WidgetKey(15, 6, "R");
static K_MIN_T: WidgetKey = WidgetKey(18, 6, "t");
static K_T: WidgetKey = WidgetKey(18, 6, "T");
static K_MIN_Y: WidgetKey = WidgetKey(21, 6, "y");
static K_Y: WidgetKey = WidgetKey(21, 6, "Y");
static K_MIN_U: WidgetKey = WidgetKey(24, 6, "u");
static K_U: WidgetKey = WidgetKey(24, 6, "U");
static K_MIN_I: WidgetKey = WidgetKey(27, 6, "i");
static K_I: WidgetKey = WidgetKey(27, 6, "I");
static K_MIN_O: WidgetKey = WidgetKey(30, 6, "o");
static K_O: WidgetKey = WidgetKey(30, 6, "O");
static K_MIN_P: WidgetKey = WidgetKey(33, 6, "p");
static K_P: WidgetKey = WidgetKey(33, 6, "P");
static K_LSQB: WidgetKey = WidgetKey(36, 6, "[");
static K_LCUB: WidgetKey = WidgetKey(36, 6, "{");
static K_RSQB: WidgetKey = WidgetKey(39, 6, "]");
static K_RCUB: WidgetKey = WidgetKey(39, 6, "}");

static K_ENTER_1: WidgetKey = WidgetKey(43, 6, "░░░░");
static K_ENTER_2: WidgetKey = WidgetKey(43, 7, "░░░░");
static K_ENTER_3: WidgetKey = WidgetKey(41, 8, "░░░░░░");
static K_ENTER: WidgetKey = WidgetKey(45, 7, "↵");

static K_DEL: WidgetKey = WidgetKey(50, 6, "DEL");
static K_END: WidgetKey = WidgetKey(54, 6, "END");
static K_PGD: WidgetKey = WidgetKey(58, 6, "PGD");
static K_K_7: WidgetKey = WidgetKey(65, 6, "7");
static K_K_8: WidgetKey = WidgetKey(68, 6, "8");
static K_K_9: WidgetKey = WidgetKey(71, 6, "9");

static K_K_PLUS_1: WidgetKey = WidgetKey(74, 6, " ");
static K_K_PLUS_2: WidgetKey = WidgetKey(74, 7, "+");
static K_K_PLUS_3: WidgetKey = WidgetKey(74, 8, " ");

static K_CAPS: WidgetKey = WidgetKey(1, 8, "CAPS");
static K_MIN_A: WidgetKey = WidgetKey(7, 8, "a");
static K_A: WidgetKey = WidgetKey(7, 8, "A");
static K_MIN_S: WidgetKey = WidgetKey(10, 8, "s");
static K_S: WidgetKey = WidgetKey(10, 8, "S");
static K_MIN_D: WidgetKey = WidgetKey(13, 8, "d");
static K_D: WidgetKey = WidgetKey(13, 8, "D");
static K_MIN_F: WidgetKey = WidgetKey(16, 8, "f");
static K_F: WidgetKey = WidgetKey(16, 8, "F");
static K_MIN_G: WidgetKey = WidgetKey(19, 8, "g");
static K_G: WidgetKey = WidgetKey(19, 8, "G");
static K_MIN_H: WidgetKey = WidgetKey(22, 8, "h");
static K_H: WidgetKey = WidgetKey(22, 8, "H");
static K_MIN_J: WidgetKey = WidgetKey(25, 8, "j");
static K_J: WidgetKey = WidgetKey(25, 8, "J");
static K_MIN_K: WidgetKey = WidgetKey(28, 8, "k");
static K_K: WidgetKey = WidgetKey(28, 8, "K");
static K_MIN_L: WidgetKey = WidgetKey(31, 8, "l");
static K_L: WidgetKey = WidgetKey(31, 8, "L");
static K_SEMICOLON: WidgetKey = WidgetKey(34, 8, ";");
static K_PARENTHESIS: WidgetKey = WidgetKey(34, 8, ":");
static K_QUOTE: WidgetKey = WidgetKey(37, 8, "'");
static K_DOUBLEQUOTE: WidgetKey = WidgetKey(37, 8, "\"");
static K_K_4: WidgetKey = WidgetKey(65, 8, "4");
static K_K_5: WidgetKey = WidgetKey(68, 8, "5");
static K_K_6: WidgetKey = WidgetKey(71, 8, "6");

static K_LSHIFT: WidgetKey = WidgetKey(1, 10, "SHIFT");
static K_MIN_Z: WidgetKey = WidgetKey(9, 10, "z");
static K_Z: WidgetKey = WidgetKey(9, 10, "Z");
static K_MIN_X: WidgetKey = WidgetKey(12, 10, "x");
static K_X: WidgetKey = WidgetKey(12, 10, "X");
static K_MIN_C: WidgetKey = WidgetKey(15, 10, "c");
static K_C: WidgetKey = WidgetKey(15, 10, "C");
static K_MIN_V: WidgetKey = WidgetKey(18, 10, "v");
static K_V: WidgetKey = WidgetKey(18, 10, "V");
static K_MIN_B: WidgetKey = WidgetKey(21, 10, "b");
static K_B: WidgetKey = WidgetKey(21, 10, "B");
static K_MIN_N: WidgetKey = WidgetKey(24, 10, "n");
static K_N: WidgetKey = WidgetKey(24, 10, "N");
static K_MIN_M: WidgetKey = WidgetKey(27, 10, "m");
static K_M: WidgetKey = WidgetKey(27, 10, "M");
static K_COMMA: WidgetKey = WidgetKey(30, 10, ",");
static K_LANB: WidgetKey = WidgetKey(30, 10, "<");
static K_PERIOD: WidgetKey = WidgetKey(33, 10, ".");
static K_RANB: WidgetKey = WidgetKey(33, 10, ">");
static K_SLASH: WidgetKey = WidgetKey(36, 10, "/");
static K_QUESTION: WidgetKey = WidgetKey(36, 10, "?");
static K_RSHIFT: WidgetKey = WidgetKey(42, 10, "SHIFT");
static K_ARROW_UP: WidgetKey = WidgetKey(54, 10, "(↑)");

static K_K_1: WidgetKey = WidgetKey(65, 10, "1");
static K_K_2: WidgetKey = WidgetKey(68, 10, "2");
static K_K_3: WidgetKey = WidgetKey(71, 10, "3");

static K_K_ENTER_1: WidgetKey = WidgetKey(74, 10, "░");
static K_K_ENTER_2: WidgetKey = WidgetKey(74, 11, "░");
static K_K_ENTER_3: WidgetKey = WidgetKey(74, 12, "░");
static K_K_ENTER: WidgetKey = WidgetKey(74, 11, "↵");

static K_LCTRL: WidgetKey = WidgetKey(1, 12, "CTRL");

static K_LWIN: WidgetKey = WidgetKey(6, 12, "WIN");

static K_LALT: WidgetKey = WidgetKey(10, 12, "ALT");
static K_SPACE: WidgetKey = WidgetKey(14, 12, "     SPACE     ");
static K_RALT: WidgetKey = WidgetKey(30, 12, "ALT");

static K_RWIN: WidgetKey = WidgetKey(34, 12, "WIN");
static K_RPROP: WidgetKey = WidgetKey(38, 12, "PROP");
static K_RCTRL: WidgetKey = WidgetKey(43, 12, "CTRL");

static K_ARROW_LEFT: WidgetKey = WidgetKey(50, 12, "(←)");
static K_ARROW_DOWN: WidgetKey = WidgetKey(54, 12, "(↓)");
static K_ARROW_RIGHT: WidgetKey = WidgetKey(58, 12, "(→)");
static K_K_0: WidgetKey = WidgetKey(65, 12, " 0  ");
static K_K_PERIOD: WidgetKey = WidgetKey(71, 12, ".");

fn draw_key(rustbox: &RustBox, widget_key: &WidgetKey, fg: Color, bg: Color) {
    rustbox.print(widget_key.0 + 2,
                  widget_key.1 + 4,
                  rustbox::RB_NORMAL,
                  fg,
                  bg,
                  widget_key.2);
}

fn draw_keys(rustbox: &RustBox, keys: &[&WidgetKey], fg: Color, bg: Color) {
    for k in keys {
        draw_key(&rustbox, k, fg, bg);
    }
}

fn draw_keyboard(rustbox: &RustBox, inputmode: &rustbox::InputMode) {

    // The 4th corners
    rustbox.print(0, 0, rustbox::RB_NORMAL, Color::White, Color::Black, "┌");
    rustbox.print(79, 0, rustbox::RB_NORMAL, Color::White, Color::Black, "┐");
    rustbox.print(0, 23, rustbox::RB_NORMAL, Color::White, Color::Black, "└");
    rustbox.print(79,
                  23,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "┘");

    // Horizontal lines
    for i in 1..79 {
        rustbox.print(i, 0, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
        rustbox.print(i, 23, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
        rustbox.print(i, 17, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
        rustbox.print(i, 4, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
    }

    // Vertical lines
    for i in 1..23 {
        rustbox.print(0, i, rustbox::RB_NORMAL, Color::White, Color::Black, "│");
        rustbox.print(79, i, rustbox::RB_NORMAL, Color::White, Color::Black, "│");
    }

    rustbox.print(0, 4, rustbox::RB_NORMAL, Color::White, Color::Black, "├");
    rustbox.print(0, 17, rustbox::RB_NORMAL, Color::White, Color::Black, "├");

    rustbox.print(79, 4, rustbox::RB_NORMAL, Color::White, Color::Black, "┤");
    rustbox.print(79,
                  17,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "┤");

    for i in 5..17 {
        rustbox.print(1,
                      i,
                      rustbox::RB_NORMAL,
                      Color::Yellow,
                      Color::Yellow,
                      "█");
        rustbox.print(78,
                      i,
                      rustbox::RB_NORMAL,
                      Color::Yellow,
                      Color::Yellow,
                      "█");
    }

    // Head
    rustbox.print(33,
                  1,
                  rustbox::RB_BOLD,
                  Color::Magenta,
                  Color::Black,
                  "Keyboard demo!");
    rustbox.print(21,
                  2,
                  rustbox::RB_NORMAL,
                  Color::Magenta,
                  Color::Black,
                  "(press CTRL+X and then CTRL+Q to exit)");
    rustbox.print(15,
                  3,
                  rustbox::RB_NORMAL,
                  Color::Magenta,
                  Color::Black,
                  "(press CTRL+X and then CTRL+C to change input mode)");

    // Body
    draw_keys(&rustbox,
              &[&K_ESC,
                &K_F1,
                &K_F2,
                &K_F3,
                &K_F4,
                &K_F5,
                &K_F6,
                &K_F7,
                &K_F8,
                &K_F9,
                &K_F10,
                &K_F11,
                &K_F12,

                &K_PRN,
                &K_SCR,
                &K_BRK,
                &K_LED1,
                &K_LED2,
                &K_LED3,

                &K_TILDE,
                &K_1,
                &K_2,
                &K_3,
                &K_4,
                &K_5,
                &K_6,
                &K_7,
                &K_8,
                &K_9,
                &K_0,
                &K_MINUS,
                &K_EQUALS,
                &K_BACKSLASH,
                &K_BACKSPACE,
                &K_INS,
                &K_HOM,
                &K_PGU,
                &K_K_NUMLOCK,
                &K_K_SLASH,
                &K_K_STAR,
                &K_K_MINUS,

                &K_TAB,

                &K_MIN_Q,
                &K_MIN_W,
                &K_MIN_E,
                &K_MIN_R,
                &K_MIN_T,
                &K_MIN_Y,
                &K_MIN_U,

                &K_MIN_I,

                &K_MIN_O,
                &K_MIN_P,
                &K_LSQB,
                &K_RSQB,

                &K_ENTER_1,
                &K_ENTER_2,
                &K_ENTER_3,

                &K_DEL,
                &K_END,
                &K_PGD,
                &K_K_7,
                &K_K_8,
                &K_K_9,

                &K_K_PLUS_1,
                &K_K_PLUS_2,
                &K_K_PLUS_3,

                &K_CAPS,
                &K_MIN_A,
                &K_MIN_S,
                &K_MIN_D,
                &K_MIN_F,
                &K_MIN_G,
                &K_MIN_H,
                &K_MIN_J,
                &K_MIN_K,
                &K_MIN_L,
                &K_SEMICOLON,
                &K_QUOTE,
                &K_K_4,
                &K_K_5,
                &K_K_6,

                &K_LSHIFT,
                &K_MIN_Z,
                &K_MIN_X,
                &K_MIN_C,
                &K_MIN_V,
                &K_MIN_B,
                &K_MIN_N,
                &K_MIN_M,
                &K_COMMA,
                &K_PERIOD,
                &K_SLASH,
                &K_RSHIFT,
                &K_ARROW_UP,
                &K_K_1,
                &K_K_2,
                &K_K_3,

                &K_K_ENTER_1,
                &K_K_ENTER_2,
                &K_K_ENTER_3,

                &K_LCTRL,

                &K_LWIN,
                &K_LALT,
                &K_SPACE,

                &K_RCTRL,
                &K_RPROP,
                &K_RWIN,
                &K_RALT,
                &K_ARROW_LEFT,
                &K_ARROW_DOWN,
                &K_ARROW_RIGHT,
                &K_K_0,
                &K_K_PERIOD],
              Color::White,
              Color::Blue);

    draw_key(&rustbox, &K_ENTER, Color::White, Color::Blue);
    draw_key(&rustbox, &K_K_ENTER, Color::White, Color::Blue);

    // Footer
    let inputmode_str = match *inputmode {
        InputMode::EscMouse => "TB_INPUT_ESC | TB_INPUT_MOUSE",
        InputMode::AltMouse => "TB_INPUT_ALT | TB_INPUT_MOUSE",
        InputMode::Esc => "TB_INPUT_ESC",
        InputMode::Alt => "TB_INPUT_ALT",
        _ => "",
    };

    rustbox.print(3,
                  18,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &format!("Input mode: {}", inputmode_str));
}

fn pretty_print_resize(rustbox: &RustBox, x: i32, y: i32) {
    rustbox.print(3,
                  19,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &*format!("Resize event: {:?} x {:?}", x, y));
}

fn pretty_print_mouse(rustbox: &RustBox,
                      mouse: Mouse,
                      x: i32,
                      y: i32,
                      mut counter: usize)
                      -> usize {
    counter += 1;
    rustbox.print(3,
                  19,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &*format!("Mouse event: {:?} x {:?}", x, y));
    rustbox.print(43,
                  19,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "Key: ");
    rustbox.print(48,
                  19,
                  rustbox::RB_NORMAL,
                  Color::Yellow,
                  Color::Black,
                  &*format!("Mouse{:?}: {:?}", mouse, counter));
    counter
}

fn pretty_print_press(rustbox: &RustBox,
                      emod: &u8,
                      combo: &HashMap<rustbox::keyboard::Key, (u16, String, Vec<WidgetKey>)>,
                      key: &Key) {
    let input_key = match combo.get(key) {
        Some(value) => {
            for v in &value.2 {
                draw_key(&rustbox, &v, Color::White, Color::Red);
                if *emod == 1 {
                    draw_key(&rustbox, &K_LALT, Color::White, Color::Red);
                    draw_key(&rustbox, &K_RALT, Color::White, Color::Red);
                }
            }
            (value.0, value.1.to_string())
        }
        None => (0, String::from("")),
    };

    let character = match key {
        &Key::Char(c) => c.to_string(),
        _ => String::from(""),
    };

    let byte_character;
    if character == "" {
        byte_character = 0;
    } else {
        byte_character = character.as_bytes()[0];
    }

    rustbox.print(3,
                  19,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "Key: ");
    rustbox.print(8,
                  19,
                  rustbox::RB_NORMAL,
                  Color::Yellow,
                  Color::Black,
                  &format!("decimal: {}", input_key.0));
    rustbox.print(8,
                  20,
                  rustbox::RB_NORMAL,
                  Color::Green,
                  Color::Black,
                  &format!("hex:     0x{:x}", input_key.0));
    rustbox.print(8,
                  21,
                  rustbox::RB_NORMAL,
                  Color::Cyan,
                  Color::Black,
                  &format!("octal:   {:07o}", input_key.0));
    rustbox.print(8,
                  22,
                  rustbox::RB_NORMAL,
                  Color::Red,
                  Color::Black,
                  &format!("string:  {}", input_key.1));
    rustbox.print(54,
                  19,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  "Char: ");
    rustbox.print(60,
                  19,
                  rustbox::RB_NORMAL,
                  Color::Yellow,
                  Color::Black,
                  &format!("decimal: {}", byte_character));
    rustbox.print(60,
                  20,
                  rustbox::RB_NORMAL,
                  Color::Green,
                  Color::Black,
                  &format!("hex:     0x{:x}", byte_character));
    rustbox.print(60,
                  21,
                  rustbox::RB_NORMAL,
                  Color::Cyan,
                  Color::Black,
                  &format!("octal:   {:04o}", byte_character));
    rustbox.print(60,
                  22,
                  rustbox::RB_NORMAL,
                  Color::Red,
                  Color::Black,
                  &format!("string:  {}", character));

    let mut modifier = "Modifier: none";
    if *emod == 1 {
        modifier = "Modifier: TB_MOD_ALT";
    }
    rustbox.print(54,
                  18,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  modifier);
}

fn main() {
    let mut combo = HashMap::new();
    let list_min_char = [('a', K_MIN_A, K_A),
                         ('b', K_MIN_B, K_B),
                         ('c', K_MIN_C, K_C),
                         ('d', K_MIN_D, K_D),
                         ('e', K_MIN_E, K_E),
                         ('f', K_MIN_F, K_F),
                         ('g', K_MIN_G, K_G),
                         ('h', K_MIN_H, K_H),
                         ('i', K_MIN_I, K_I),
                         ('j', K_MIN_J, K_J),
                         ('k', K_MIN_K, K_K),
                         ('l', K_MIN_L, K_L),
                         ('m', K_MIN_M, K_M),
                         ('n', K_MIN_N, K_N),
                         ('o', K_MIN_O, K_O),
                         ('p', K_MIN_P, K_P),
                         ('q', K_MIN_Q, K_Q),
                         ('r', K_MIN_R, K_R),
                         ('s', K_MIN_S, K_S),
                         ('t', K_MIN_T, K_T),
                         ('u', K_MIN_U, K_U),
                         ('v', K_MIN_V, K_V),
                         ('w', K_MIN_W, K_W),
                         ('x', K_MIN_X, K_X),
                         ('y', K_MIN_Y, K_Y),
                         ('z', K_MIN_Z, K_Z)];
    let list_maj_char = [('A', K_A),
                         ('B', K_B),
                         ('C', K_C),
                         ('D', K_D),
                         ('E', K_E),
                         ('F', K_F),
                         ('G', K_G),
                         ('H', K_H),
                         ('I', K_I),
                         ('J', K_J),
                         ('K', K_K),
                         ('L', K_L),
                         ('M', K_M),
                         ('N', K_N),
                         ('O', K_O),
                         ('P', K_P),
                         ('Q', K_Q),
                         ('R', K_R),
                         ('S', K_S),
                         ('T', K_T),
                         ('U', K_U),
                         ('V', K_V),
                         ('W', K_W),
                         ('X', K_X),
                         ('Y', K_Y),
                         ('Z', K_Z),
                         ('=', K_EQUALS),
                         ('\\', K_BACKSLASH)];

    for c in list_min_char.iter() {
        combo.insert(Key::Char(c.0),
                     (0, String::from("CTRL+2, CTRL+~"), vec![c.1]));
    }
    for c in list_maj_char.iter() {
        combo.insert(Key::Char(c.0),
                     (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, c.1]));
    }
    for c in list_min_char.iter() {
        combo.insert(Key::Ctrl(c.0),
                     (0,
                      format!("CTRL+{}", c.0.to_ascii_uppercase()),
                      vec![K_LCTRL, K_RCTRL, c.2]));
    }
    combo.insert(Key::Char('"'),
                 (0, String::from("CTRL+3, ESC, CTRL+["), vec![K_DOUBLEQUOTE, K_LSHIFT, K_RSHIFT]));
    combo.insert(Key::Char('~'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_TILDE_SHIFT]));
    combo.insert(Key::Char('!'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_1_SHIFT]));
    combo.insert(Key::Char('@'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_2_SHIFT]));
    combo.insert(Key::Char('#'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_3_SHIFT]));
    combo.insert(Key::Char('$'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_4_SHIFT]));
    combo.insert(Key::Char('%'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_5_SHIFT]));
    combo.insert(Key::Char('^'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_6_SHIFT]));
    combo.insert(Key::Char('&'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_7_SHIFT]));
    combo.insert(Key::Char('*'),
                 (0,
                  String::from("CTRL+2, CTRL+~"),
                  vec![K_LSHIFT, K_RSHIFT, K_8_SHIFT, K_K_STAR]));
    combo.insert(Key::Char('('),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_9_SHIFT]));
    combo.insert(Key::Char(')'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_0_SHIFT]));

    combo.insert(Key::Char('-'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_MINUS, K_K_MINUS]));
    combo.insert(Key::Char('_'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_MINUS_SHIFT]));
    combo.insert(Key::Char('+'),
                 (0,
                  String::from("CTRL+2, CTRL+~"),
                  vec![K_LSHIFT, K_RSHIFT, K_EQUALS_SHIFT, K_K_PLUS_1, K_K_PLUS_2, K_K_PLUS_3]));
    combo.insert(Key::Char('|'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_BACKSLASH_SHIFT]));
    combo.insert(Key::Char('{'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_LCUB]));
    combo.insert(Key::Char('}'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_RCUB]));
    combo.insert(Key::Char(':'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_PARENTHESIS]));
    combo.insert(Key::Char('>'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_RANB]));
    combo.insert(Key::Char('<'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_LANB]));
    combo.insert(Key::Char('?'),
                 (0, String::from("CTRL+2, CTRL+~"), vec![K_LSHIFT, K_RSHIFT, K_QUESTION]));
    combo.insert(Key::Ctrl('h'),
                 (8, String::from("CTRL+H, BACKSPACE"), vec![K_LCTRL, K_RCTRL, K_H, K_BACKSPACE]));
    combo.insert(Key::Tab,
                 (9, String::from("CTRL + I, TAB"), vec![K_LCTRL, K_RCTRL, K_I, K_TAB]));
    combo.insert(Key::Enter,
                 (13,
                  String::from("CTRL+M, ENTER"),
                  vec![K_LCTRL,
                       K_RCTRL,
                       K_ENTER_1,
                       K_ENTER_2,
                       K_ENTER_3,
                       K_ENTER,
                       K_M,
                       K_K_ENTER_1,
                       K_K_ENTER_2,
                       K_K_ENTER_3,
                       K_K_ENTER]));
    combo.insert(Key::Esc,
                 (27,
                  String::from("CTRL+3, ESC, CTRL+["),
                  vec![K_LCTRL, K_RCTRL, K_ESC, K_3, K_LSQB]));
    combo.insert(Key::Ctrl('\\'),
                 (28, String::from("CTRL+4, CTRL+\\"), vec![K_LCTRL, K_RCTRL, K_4, K_BACKSLASH]));
    combo.insert(Key::Ctrl(']'),
                 (29, String::from("CTRL+5, CTRL+]"), vec![K_LCTRL, K_RCTRL, K_5, K_RSQB]));
    combo.insert(Key::Ctrl('6'),
                 (30, String::from("CTRL+6"), vec![K_LCTRL, K_RCTRL, K_6]));
    combo.insert(Key::Ctrl('/'),
                 (31,
                  String::from("CTRL+7, CTRL+/, CTRL+_"),
                  vec![K_LCTRL, K_RCTRL, K_7, K_K_SLASH, K_MINUS_SHIFT]));
    combo.insert(Key::Char(' '), (32, String::from("SPACE"), vec![K_SPACE]));
    combo.insert(Key::Backspace,
                 (127,
                  String::from("CTRL+8, BACKSPACE 2"),
                  vec![K_LCTRL, K_RCTRL, K_8, K_BACKSPACE]));

    combo.insert(Key::Right,
                 (65514, String::from("ARROW RIGHT"), vec![K_ARROW_RIGHT]));
    combo.insert(Key::Left,
                 (65515, String::from("ARROW LEFT"), vec![K_ARROW_LEFT]));
    combo.insert(Key::Down,
                 (65516, String::from("ARROW DOWN"), vec![K_ARROW_DOWN]));
    combo.insert(Key::Up, (65517, String::from("ARROW UP"), vec![K_ARROW_UP]));

    combo.insert(Key::F(1), (65535, String::from("F1"), vec![K_F1]));
    combo.insert(Key::F(2), (65534, String::from("F2"), vec![K_F2]));
    combo.insert(Key::F(3), (65533, String::from("F3"), vec![K_F3]));
    combo.insert(Key::F(4), (65532, String::from("F4"), vec![K_F4]));
    combo.insert(Key::F(5), (65531, String::from("F5"), vec![K_F5]));
    combo.insert(Key::F(6), (65530, String::from("F6"), vec![K_F6]));
    combo.insert(Key::F(7), (65529, String::from("F7"), vec![K_F7]));
    combo.insert(Key::F(8), (65528, String::from("F8"), vec![K_F8]));
    combo.insert(Key::F(9), (65527, String::from("F9"), vec![K_F9]));
    combo.insert(Key::F(10), (65526, String::from("F10"), vec![K_F10]));
    combo.insert(Key::F(11), (65525, String::from("F11"), vec![K_F11]));
    combo.insert(Key::F(12), (65524, String::from("F12"), vec![K_F12]));

    combo.insert(Key::Insert, (65523, String::from("INSERT"), vec![K_INS]));
    combo.insert(Key::Delete, (65522, String::from("DELETE"), vec![K_DEL]));
    combo.insert(Key::Home, (65521, String::from("HOME"), vec![K_HOM]));
    combo.insert(Key::End, (65520, String::from("END"), vec![K_END]));
    combo.insert(Key::PageUp, (65519, String::from("PGU"), vec![K_PGU]));
    combo.insert(Key::PageDown, (65518, String::from("PGD"), vec![K_PGD]));

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let chmap: [InputMode; 4] = [InputMode::EscMouse,
                                 InputMode::AltMouse,
                                 InputMode::Esc,
                                 InputMode::Alt];
    let mut counter: usize = 0;
    let mut inputmode: usize = 0;
    let mut ctrl_xpressed: bool = false;

    rustbox.set_input_mode(InputMode::EscMouse);
    rustbox.clear();
    draw_keyboard(&rustbox, &chmap[inputmode]);
    rustbox.present();

    loop {
        match rustbox.poll_event(true) {
            Ok(rustbox::Event::KeyEventRaw(emod, ekey, ech)) => {
                let k = match ekey {
                    0 => char::from_u32(ech).map(|c| Key::Char(c)),
                    a => Key::from_code(a),
                };
                if let Some(key) = k {
                    if key == Key::Ctrl('q') && ctrl_xpressed {
                        break;
                    }
                    if key == Key::Ctrl('c') && ctrl_xpressed {
                        inputmode += 1;
                        if inputmode >= 4 {
                            inputmode = 0;
                        }
                        rustbox.set_input_mode(chmap[inputmode]);
                    }
                    if key == Key::Ctrl('x') {
                        ctrl_xpressed = true;
                    } else {
                        ctrl_xpressed = false;
                    }
                    rustbox.clear();
                    draw_keyboard(&rustbox, &chmap[inputmode]);
                    pretty_print_press(&rustbox, &emod, &combo, &key);
                    rustbox.present();
                }
            }
            Ok(rustbox::Event::ResizeEvent(x, y)) => {
                rustbox.clear();
                draw_keyboard(&rustbox, &chmap[inputmode]);
                pretty_print_resize(&rustbox, x, y);
                rustbox.present();
            }
            Ok(rustbox::Event::MouseEvent(mouse, x, y)) => {
                rustbox.clear();
                draw_keyboard(&rustbox, &chmap[inputmode]);
                counter = pretty_print_mouse(&rustbox, mouse, x, y, counter);
                rustbox.present();
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
