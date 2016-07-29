# Termion

Termion is a pure Rust, bindless library for low-level handling, manipulating
and reading information about terminals. This provides a full-featured
alternative to Termbox.

Termion aims to be simple and yet expressive. It is bindless, meaning that it
is not a front-end to some other library (e.g., ncurses or termbox), but a
standalone library directly talking to the TTY.

Termion is quite convinient, due to its complete coverage of essential TTY
features, providing one consistent API. Termion is rather low-level containing
only abstraction aligned with what actually happens behind the scenes, for
something more high-level, refer to inquirer-rs, which uses Termion as backend.

Termion generates escapes and API calls for the user. This makes it a whole lot
cleaner to use escapes.

Supports Redox, Mac OS X, BSD, and Linux (or, in general, ANSI terminals).

[Documentation.](http://ticki.github.io/termion/) | [Examples.](https://github.com/Ticki/termion/tree/master/examples) | [Changelog.](https://github.com/Ticki/termion/tree/master/CHANGELOG.md)

## A note on stability

Although small breaking changes might happen, I will try my best to avoid them,
and this crate can generally be considered stable.

## Cargo.toml

```toml
[dependencies]
termion = "1.0.3"
```

## 0.1.0 to 1.0.0 guide

This sample table gives an idea of how to go bu converting to the new major
version of Termion.

| 0.1.0                          | 1.0.0
|--------------------------------|---------------------------
| `use termion::IntoRawMode`     | `use termion::raw::IntoRawMode`
| `stdout.color(color::Red);`    | `write!(stdout, "{}", color::Fg(color::Red));`
| `stdout.color_bg(color::Red);` | `write!(stdout, "{}", color::Bg(color::Red));`
| `stdout.goto(x, y);`           | `write!(stdout, "{}", cursor::Goto(x, y));`
| `color::rgb(r, g, b);`         | `color::Rgb(r, g, b)` (truecolor)
| `x.with_mouse()`               | `MouseTerminal::from(x)`

## Features

- Raw mode.
- TrueColor.
- 256-color mode.
- Cursor movement.
- Text formatting.
- Console size.
- TTY-only stream.
- Control sequences.
- Termios control.
- Password input.
- Redox support.
- Safe `isatty` wrapper.
- Panic-free error handling.
- Special keys events (modifiers, special keys, etc.).
- Allocation-free.
- Asynchronous key events.
- Mouse input
- Carefully tested.

and much more.

## Examples

### Style and colors.

```rust
extern crate termion;

use termion::{color, style};

use std::io;

fn main() {
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic", style::Italic);
}
```

### Moving the cursor

```rust
extern crate termion;

fn main() {
    print!("{}{}Stuff", termion::clear::All, termion::cursor::Goto(1, 1));
}

```

### Mouse

```rust
extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
```

### Read a password

```rust
extern crate termion;

use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write(b"password: ").unwrap();
    stdout.flush().unwrap();

    let pass = stdin.read_passwd(&mut stdout);

    if let Ok(Some(pass)) = pass {
        stdout.write(pass.as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
    } else {
        stdout.write(b"Error\n").unwrap();
    }
}
```

## Usage

See `examples/`, and the documentation, which can be rendered using `cargo doc`.

For a more complete example, see [a minesweeper implementation](https://github.com/redox-os/games-for-redox/blob/master/src/minesweeper/main.rs), that I made for Redox using termion.

<img src="image.png" width="200">

## License

MIT/X11.
