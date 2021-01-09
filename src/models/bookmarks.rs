use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    // tags: Vec<String>,
}

impl Bookmark {
    pub fn create(url: String) -> Result<Bookmark, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        conn.execute(
            "create table if not exists bookmarks (
                 id integer primary key,
                 url text not null
             )",
            NO_PARAMS,
        )?;

        let bmark = Bookmark {
            id: conn.last_insert_rowid() + 1,
            url: url,
        };

        conn.execute(
            "INSERT INTO bookmarks (url) VALUES (?1)",
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
    pub fn new() -> Result<Bookmarks, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let mut stmt = conn.prepare("SELECT id, url FROM bookmarks")?;
        let collector: Vec::<String> = vec![];
        let bmarks_iter = stmt.query_map(collector.iter(), |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
            })
        })?;

        let mut bmarks = vec![];
        for bmark in bmarks_iter {
            bmarks.push(bmark.unwrap());
        };

        Ok(Bookmarks { items: bmarks, highlighted_item_idx: 0 })
    }
    
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.items.push(bookmark);
    }

    pub fn collect_urls(&self) -> Vec<String> {
        self.items.iter().map(|bookmark| bookmark.url()).collect()
    }
}