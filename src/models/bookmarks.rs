use rusqlite::{params, Connection, Result};

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    // tags: Vec<String>,
}

impl Bookmark {
    pub fn create(url: String) -> Result<Bookmark, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let bmark = Bookmark {
            id: conn.last_insert_rowid() + 1,
            url: url,
        };

        conn.execute(
            "INSERT INTO bookmarks (url) values (?1)",
            params![bmark.url],
        )?;

        Ok(bmark)
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
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