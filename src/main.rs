mod ui;
mod app;

use std::io::{
    Write,
    stdout,
    stdin,
    Error
};
use termion::{
    event::{
        Key,
        Event,
    },
    input::{
        TermRead,
        MouseTerminal
    },
    raw::IntoRawMode,
};

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut app = app::App::new();

    loop {
        ui::draw();

        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Esc) => break,
                Event::Key(Key::Char(c)) => { app.add_char(c) },
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}

// struct Bookmark {
//     title: String,
//     tags: Vec<String>,
//     url: String,
// }

// impl Bookmark {
//     fn new(title: String, tags: Vec<String>, url: String) -> Bookmark {
//         Bookmark {
//             title,
//             tags,
//             url
//         }
//     }
// }