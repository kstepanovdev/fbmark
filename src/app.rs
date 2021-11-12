use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use tui::widgets::ListState;

use crate::models::bookmarks::Bookmark;
use crate::tagpacker_adapter;

use rusqlite::{params, Connection};

pub struct App {
    pub current_mode: Mode,
    pub search_string: String,
    pub new_bookmark_name: String,
    pub bookmarks: Vec<Bookmark>,
    pub filtered_bookmarks: Vec<Bookmark>,
    pub bookmarks_state: ListState,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>) -> App {
        App {
            current_mode: Mode::Search,
            search_string: String::from(""),
            new_bookmark_name: String::from(""),
            filtered_bookmarks: bookmarks.clone(),
            bookmarks: bookmarks,
            bookmarks_state: ListState::default(),
        }
    }
    pub fn add_char(&mut self, c: char) {
        self.select_field().push(c)
    }
    pub fn remove_char(&mut self) {
        self.select_field().pop();
    }
    pub fn wipe_line(&mut self) {
        *self.select_field() = String::from("");
    }
    pub fn on_up(&mut self) {
        if self.filtered_bookmarks.len() == 0 {
            return self.bookmarks_state.select(None);
        };

        let i = match self.bookmarks_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_bookmarks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.bookmarks_state.select(Some(i));
    }

    pub fn on_down(&mut self) {
        if self.filtered_bookmarks.len() == 0 {
            return self.bookmarks_state.select(None);
        };

        let i = match self.bookmarks_state.selected() {
            Some(i) => {
                if i >= self.filtered_bookmarks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.bookmarks_state.select(Some(i));
    }

    pub fn on_delete(&mut self) {
        if self.filtered_bookmarks.len() == 0 || self.bookmarks_state.selected() == None {
            return;
        }

        match self.bookmarks_state.selected() {
            Some(index) => {
                let id_for_deletion = self.filtered_bookmarks[index].id.clone();
                let conn = Connection::open("fbmark.db").unwrap();
                conn.execute(
                    "DELETE FROM bookmarks WHERE id=?1",
                    params![id_for_deletion],
                )
                .unwrap();

                self.bookmarks = Bookmark::collect_all().unwrap();
                self.filtered_bookmarks.remove(index);
                if self.filtered_bookmarks.len() == 0 {
                    self.bookmarks_state.select(None)
                } else {
                    self.on_down()
                }
            }
            None => panic!("Index not found"),
        };
    }

    pub fn resolve_enter(&mut self) {
        match self.current_mode {
            Mode::Search => match self.bookmarks_state.selected() {
                Some(index) => {
                    let url = self.filtered_bookmarks[index].url();
                    open::that(url).unwrap();
                }
                None => {}
            },
            Mode::AddBookmark => {
                let bmark_name = self.new_bookmark_name.clone();
                match Bookmark::create(bmark_name) {
                    Ok(bmark) => self.bookmarks.push(bmark),
                    Err(e) => panic!("{}", e),
                }
                self.new_bookmark_name = "".to_string();
                self.current_mode = Mode::Search;
            }
        }
    }

    pub fn select_field(&mut self) -> &mut String {
        match self.current_mode {
            Mode::Search => &mut self.search_string,
            Mode::AddBookmark => &mut self.new_bookmark_name,
        }
    }

    pub fn change_mode(&mut self) {
        match self.current_mode {
            Mode::Search => self.current_mode = Mode::AddBookmark,
            Mode::AddBookmark => self.current_mode = Mode::Search,
        }
    }

    pub fn paste_from_clipboard(&mut self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(payload) => {
                for c in payload.chars() {
                    self.select_field().push(c);
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        };
    }

    pub fn sync_bmarks(&mut self) {
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Search,
    AddBookmark,
}

pub enum Event<I> {
    Input(I),
}
