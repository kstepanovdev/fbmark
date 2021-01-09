mod ui;
mod app; 
mod models;

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

use models::bookmarks::{Bookmark, Bookmarks};

use app::App;

use tui::Terminal;
use tui::backend::TermionBackend;

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut bookmarks = Bookmarks::new();

    match Bookmark::create(String::from("https://lichess.org/")) {
        Ok(bmark) => bookmarks.add_bookmark(bmark),
        Err(e) => panic!(e)
    }
    match Bookmark::create(String::from("https://github.com/")) {
        Ok(bmark) => bookmarks.add_bookmark(bmark),
        Err(e) => panic!(e)
    }

    let mut app = App::new(bookmarks);

    ui::draw(&mut app, &mut terminal);

    // loop {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => {
                    terminal.clear()?;
                    break
                },
                Key::Backspace => app.remove_char(),
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                Key::Char('\n') => app.resolve_enter(),
                Key::Ctrl('u') => app.wipe_line(),
                Key::Ctrl('v') => app.paste_from_clipboard(),
                Key::Char('`') => { app.change_mode() },
                Key::Char(c) => { app.add_char(c) },
                _ => {}
            }
            ui::draw(&mut app, &mut terminal);
        }
    // }

    Ok(())
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