use rusqlite::{params, Connection, Result, NO_PARAMS};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub id: Uuid,
    pub url: String,
}

impl Bookmark {
    pub fn create(url: String) -> Result<Bookmark, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        conn.execute(
            "create table if not exists bookmarks (
                 id text primary key,
                 url text not null
             )",
            NO_PARAMS,
        )?;

        let bmark = Bookmark {
            id: Uuid::new_v4(),
            url,
        };

        conn.execute(
            "INSERT INTO bookmarks (url) VALUES (?1)",
            params![bmark.url],
        )?;

        Ok(bmark)
    }

    pub fn collect_all() -> Result<Vec<Bookmark>, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let mut stmt = conn.prepare("SELECT id, url FROM bookmarks")?;
        let collector: Vec<String> = vec![];
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

        Ok(bmarks)
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}