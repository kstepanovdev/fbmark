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
use app::App;
use app::{Bookmark, Bookmarks};

use tui::Terminal;
use tui::backend::TermionBackend;

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut app = App::new();

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut bmarks = Bookmarks::new();
    bmarks.add_bookmark(Bookmark::new(String::from("https://lichess.org/"))); 
    bmarks.add_bookmark(Bookmark::new(String::from("https://github.com/")));
    
    ui::draw(&mut app, &mut terminal, &bmarks);

    // loop {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => break,
                Key::Backspace => app.remove_char(),
                Key::Ctrl('u') => app.wipe_line(),
                Key::Char(c) => { app.add_char(c) },
                _ => {}
            }
            ui::draw(&mut app, &mut terminal, &bmarks);
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