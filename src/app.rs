use open;
use clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use crate::models::bookmarks::Bookmark;
use rusqlite::{params, Connection};


pub struct App {
    pub current_mode: Mode,
    pub search_string: String,
    pub new_bookmark_name: String,
    pub bookmarks: Vec<Bookmark>,
    pub filtered_bookmarks: Vec<Bookmark>,
    pub selected_bookmark_idx: usize,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>) -> App {
        App {
            current_mode: Mode::Search,
            search_string: String::from(""),
            new_bookmark_name: String::from(""),
            filtered_bookmarks: bookmarks.clone(),
            bookmarks: bookmarks,
            selected_bookmark_idx: 0
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
        if self.filtered_bookmarks.len() > 0 && self.selected_bookmark_idx > 0  {
            self.selected_bookmark_idx -= 1;
        }
    }
    pub fn on_down(&mut self) {
        if self.filtered_bookmarks.len() > 0 && self.selected_bookmark_idx < self.filtered_bookmarks.len() - 1 {
            self.selected_bookmark_idx += 1;
        }
    }
    pub fn on_delete(&mut self) {
        if self.filtered_bookmarks.len() == 0 {
            return
        }

        let id_for_delete = &self.filtered_bookmarks[self.selected_bookmark_idx].id;
        let conn = Connection::open("fbmark.db").unwrap();

        conn.execute(
            "DELETE FROM bookmarks WHERE id=?1",
            params![id_for_delete],
        ).unwrap();

        self.bookmarks = Bookmark::collect_all().unwrap();
        self.filtered_bookmarks.remove(self.selected_bookmark_idx);

        if self.selected_bookmark_idx != 0 && self.selected_bookmark_idx >= self.filtered_bookmarks.len() {
            self.selected_bookmark_idx -= 1;
        }
    }
    pub fn resolve_enter(&mut self) {
        match self.current_mode {
            Mode::Search => {
                let url = self.filtered_bookmarks[self.selected_bookmark_idx].url();
                open::that(url).unwrap();
            }
            Mode::AddBookmark => {
                let bmark_name = self.new_bookmark_name.clone();
                match Bookmark::create(bmark_name) {
                    Ok(bmark) => self.bookmarks.push(bmark),
                    Err(e) => panic!(e)
                }
                self.new_bookmark_name = "".to_string();
                self.current_mode = Mode::Search;
            }
        }
    }

    pub fn select_field(&mut self) -> &mut String {
        match self.current_mode {
            Mode::Search => { &mut self.search_string },
            Mode::AddBookmark => { &mut self.new_bookmark_name },
        }
    }

    pub fn change_mode(&mut self) {
        match self.current_mode {
            Mode::Search => { self.current_mode = Mode::AddBookmark },
            Mode::AddBookmark => { self.current_mode = Mode::Search },
        }
    }

    pub fn paste_from_clipboard(&mut self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(payload) => {
                for c in payload.chars() {
                    self.select_field().push(c);
                }
            },
            Err(e) => {
                // panic!(e)
            },
        };
    }
}

pub enum Mode {
    Search,
    AddBookmark
}

pub enum Event<I> {
    Input(I),
}