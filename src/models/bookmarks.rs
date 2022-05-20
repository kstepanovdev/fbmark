use rusqlite::{params, Connection, Result, NO_PARAMS};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
}

impl Bookmark {
    pub fn initialize() -> Result<(), rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        conn.execute(
            "create table if not exists bookmarks (
                 id text primary key not null unique,
                 url text not null,
                 title next
             )",
            NO_PARAMS,
        ).unwrap();
        Ok(())
    }

    pub fn create(url: String, title: String) -> Result<Bookmark, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let bmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            url,
            title,
        };

        conn.execute(
            "INSERT INTO bookmarks (id, url, title) VALUES (?1, ?2, ?3)",
            params![bmark.id, bmark.url, bmark.title],
        )?;

        Ok(bmark)
    }

    pub fn collect_all() -> Result<Vec<Bookmark>, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let mut stmt = conn.prepare("SELECT id, url, title FROM bookmarks")?;
        let collector: Vec<String> = vec![];
        let bmarks_iter = stmt.query_map(collector.iter(), |row| {
            let extracted_uuid = row.get(0)?;
            let extracted_url = row.get(1)?;
            let extracted_title = row.get(2)?;
            Ok(Bookmark {
                id: extracted_uuid,
                url: extracted_url,
                title: extracted_title,
            })
        })?;

        let mut bmarks = vec![];
        for bmark in bmarks_iter {
            bmarks.push(bmark.unwrap());
        }

        Ok(bmarks)
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}
