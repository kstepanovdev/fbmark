pub struct App {
    pub search_string: String,
    pub bookmarks: Bookmarks,
}

impl App {
    pub fn new(bookmarks: Bookmarks) -> App {
        App {
           search_string: String::from(""),
           bookmarks: bookmarks,
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