pub struct App {
    pub search_string: String,
    pub bookmarks: Bookmarks
}

impl App {
    pub fn new(bookmarks: Bookmarks) -> App {
        App {
           search_string: String::from(""),
           bookmarks: bookmarks
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
    pub items: Vec<Bookmark>
}

impl Bookmarks {
    pub fn new() -> Bookmarks {
        Bookmarks { items: Vec::new() }
    }
    
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.items.push(bookmark);
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
}


