use rusqlite::{params, Connection, Result, NO_PARAMS};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
}

impl Bookmark {
    pub fn create(url: String) -> Result<Bookmark, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        conn.execute(
            "create table if not exists bookmarks (
                 id text primary key not null,
                 url text not null
             )",
            NO_PARAMS,
        )?;

        let bmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            url,
        };

        conn.execute(
            "INSERT INTO bookmarks (id, url) VALUES (?1, ?2)",
            params![bmark.id, bmark.url],
        )?;

        Ok(bmark)
    }

    pub fn collect_all() -> Result<Vec<Bookmark>, rusqlite::Error> {
        let conn = Connection::open("fbmark.db")?;

        let mut stmt = conn.prepare("SELECT id, url FROM bookmarks")?;
        let collector: Vec<String> = vec![];
        let bmarks_iter = stmt.query_map(collector.iter(), |row| {
            let extracted_uuid = row.get(0)?;
            let extracted_url = row.get(1)?;
            Ok(Bookmark {
                id: extracted_uuid,
                url: extracted_url,
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