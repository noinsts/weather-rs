use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result};

/// Shared database connection type.
pub type Db = Arc<Mutex<Connection>>;

/// Initialize the SQLite database.
///
/// This function opens a SQLite database at the given `path`
/// and ensures that the `users` table exists. If the table does not exists,
/// it will be created with the following fields:
/// - `id` - Telegram user ID
/// - `city` - Names of the city (not null text)
/// - `created_at` - Timestamp when the record was created
/// - `updated_at` - Timestamp when the record was updated
///
/// # Arguments
/// - `path` - The file path to the SQLite database.
///
/// # Returns
/// - `Result<Db>` - A thread-safe database connection wrapped in `Arc<Mutex<Connection>>`
pub fn init_db(path: &str) -> Result<Db> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            city TEXT NOT NULL, \
            created_at TEXT NOT NULL, \
            updated_at TEXT NOT NULL\
        )",
        []
    )?;

    Ok(Arc::new(Mutex::new(conn)))
}