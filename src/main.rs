mod app;
mod models;
mod tagpacker_adapter;
mod ui;

use app::App;
use std::io::{stdin, stdout, Error};
use termion::{
    event::{Key},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

use tui::backend::TermionBackend;
use tui::Terminal;

use models::bookmarks::Bookmark;

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    match Bookmark::initialize() {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let bookmarks = match Bookmark::collect_all() {
        Ok(bmarks) => bmarks,
        Err(e) => panic!("{}", e),
    };
    let mut app = App::new(bookmarks);

    match ui::draw(&mut app, &mut terminal) {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
                terminal.clear()?;
                break;
            }
            Key::Backspace => app.remove_char(),
            Key::Up => app.on_up(),
            Key::Down => app.on_down(),
            Key::Delete => app.on_delete(),
            Key::Char('\n') => app.resolve_enter(),
            Key::Ctrl('u') => app.wipe_line(),
            Key::Ctrl('v') => app.paste_from_clipboard(),
            Key::Char('`') => app.change_mode(),
            Key::Char(c) => app.add_char(c),
            Key::F(5) => app.sync_bmarks(),
            _ => {}
        }
        ui::draw(&mut app, &mut terminal);
    }

    Ok(())
}
