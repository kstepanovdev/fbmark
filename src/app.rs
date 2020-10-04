pub struct App {
    pub search_string: String
}

impl App {
    pub fn new() -> App {
        App {
           search_string: String::from(""),
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
    pub bookmarks: Vec<Bookmark>
}

impl Bookmarks {
    pub fn new(bookmarks: Vec<Bookmark>) -> Bookmarks {
        Bookmarks {
            bookmarks
        }
    }
}

pub struct Bookmark {
    pub url: String
}

impl Bookmark {
    pub fn new(url: String) -> Bookmark {
        Bookmark {
            url
        }
    }
}


