extern crate open;
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub struct App {
    pub current_mode: Mode,
    pub search_string: String,
    pub new_bookmark_name: String,
    pub bookmarks: Bookmarks,
    pub filtered_bookmarks: Vec<Bookmark>,
    pub selected_bookmark_idx: usize,
}

impl App {
    pub fn new(bookmarks: Bookmarks) -> App {
        App {
            current_mode: Mode::Search,
            search_string: String::from(""),
            new_bookmark_name: String::from(""),
            filtered_bookmarks: bookmarks.items.clone(),
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
        if self.selected_bookmark_idx > 0 {
            self.selected_bookmark_idx -= 1;
        }
    }
    pub fn on_down(&mut self) {
        if self.selected_bookmark_idx != self.filtered_bookmarks.len() - 1 {
            self.selected_bookmark_idx += 1;
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
                self.new_bookmark_name = "".to_string();
                self.bookmarks.add_bookmark(Bookmark::new(bmark_name));
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
                // return Err(e)
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

pub struct Bookmarks {
    pub items: Vec<Bookmark>,
    pub highlighted_item_idx: isize
}

impl Bookmarks {
    pub fn new() -> Bookmarks {
        Bookmarks { items: Vec::new(), highlighted_item_idx: 0 }
    }
    
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.items.push(bookmark);
    }

    pub fn collect_urls(&self) -> Vec<String> {
        self.items.iter().map(|bookmark| bookmark.url()).collect()
    }
}

#[derive(Clone)]
pub struct Bookmark {
    pub url: String,
    tags: Vec<String>,
}

impl Bookmark {
    pub fn new(url: String) -> Bookmark {
        Bookmark {
            url,
            tags: Vec::new()
        }
    }
    pub fn url(&self) -> String {
        self.url.clone()
    }
}