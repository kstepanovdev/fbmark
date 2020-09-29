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
}

pub enum Event<I> {
    Input(I),
} 