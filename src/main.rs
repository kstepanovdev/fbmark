mod adapters;
mod app;
mod config;
mod models;
mod ui;

use app::App;
use config::Config;
use std::io::{stdin, stdout, Error};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};
use ui::{core, login};

use tui::backend::TermionBackend;
use tui::Terminal;

use models::bookmarks::Bookmark;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let stdin = stdin();
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    // user_id greeting window
    let mut cfg: Config = confy::load("fbmark").unwrap_or_default();
    login::run(&mut cfg, &mut terminal);

    // core app
    if let Err(e) = Bookmark::initialize() {
        panic!("{}", e)
    }

    let bookmarks = match Bookmark::collect_all() {
        Ok(bmarks) => bmarks,
        Err(e) => panic!("{}", e),
    };

    let mut app = App::new(bookmarks);
    let mut keys = stdin.keys();

    loop {
        core::draw(&mut app, &mut terminal).unwrap();
        let key = keys.next().unwrap();

        match key {
            Ok(Key::Esc) => {
                terminal.clear()?;
                break;
            }
            Ok(Key::Backspace) => app.remove_char(),
            Ok(Key::Up) => app.on_up(),
            Ok(Key::Down) => app.on_down(),
            Ok(Key::Delete) => app.on_delete(),
            Ok(Key::Char('\n')) => app.resolve_enter(),
            Ok(Key::Ctrl('u')) => app.wipe_line(),
            Ok(Key::Ctrl('v')) => app.paste_from_clipboard(),
            Ok(Key::Char('`')) => app.change_mode(),
            Ok(Key::Char(c)) => app.add_char(c),
            Ok(Key::F(5)) => app.sync_bmarks(),
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }

    Ok(())
}
