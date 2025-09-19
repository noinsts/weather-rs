use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result};

pub type Db = Arc<Mutex<Connection>>;

pub fn init_db(path: &str) -> Result<Db> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            city TEXT, \
            created_at TEXT NOT NULL, \
            updated_at TEXT NOT NULL\
        )",
        []
    )?;

    Ok(Arc::new(Mutex::new(conn)))
}