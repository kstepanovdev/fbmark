extern crate open;

pub struct App {
    pub search_string: String,
    pub bookmarks: Bookmarks,
    pub filtered_bookmarks: Vec<Bookmark>,
    pub selected_bookmark_idx: usize,
}

impl App {
    pub fn new(bookmarks: Bookmarks) -> App {
        App {
           search_string: String::from(""),
            filtered_bookmarks: bookmarks.items.clone(),
            bookmarks: bookmarks,
            selected_bookmark_idx: 0
        }
    }
    pub fn add_char(&mut self, c: char) {
        self.search_string.push(c);
    }
    pub fn remove_char(&mut self) {
        self.search_string.pop();
    }
    pub fn wipe_line(&mut self) {
        self.search_string = String::from("");
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
    pub fn open_bookmark(&self) {
        let url = self.filtered_bookmarks[self.selected_bookmark_idx].url();
        open::that(url).unwrap();
    }
}

pub enum Event<I> {
    Input(I),
} 

pub struct Bookmarks {
    pub items: Vec<Bookmark>,
    pub selected_item_idx: isize
}

impl Bookmarks {
    pub fn new() -> Bookmarks {
        Bookmarks { items: Vec::new(), selected_item_idx: 0 }
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