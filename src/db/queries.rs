use crate::db::db::Db;
use rusqlite::params;

pub fn user_exists(db: &Db, user_id: i64) -> bool {
    let conn = db.lock().unwrap();
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = ?1)",
            params![user_id],
            |row| row.get(0),
        )
        .unwrap_or(false);
    exists
}