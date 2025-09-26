use chrono::Utc;
use rusqlite::params;

use crate::db::db::Db;

/// Checks if a user exists in the `users` table
///
/// # Arguments
/// * `db` - Reference to the database connection wrapper.
/// * `user_id` - Telegram user ID.
///
/// Returns `true` if the user exists, otherwise `false`
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

/// Retrieves the city associated with a user.
///
/// # Arguments
/// * `db` - Reference to the database connection wrapper.
/// * `user_id` - Telegram user ID.
///
/// Returns `Some(city)` if the user has a city, otherwise `None`.
pub fn get_city(db: &Db, user_id: i64) -> Option<String> {
    let conn = db.lock().unwrap();
    let result: rusqlite::Result<String> = conn.query_row(
        "SELECT city FROM users WHERE id = ?1",
        params![user_id],
        |row| row.get(0),
    );
    result.ok()
}

/// Inserts a new user or updates the city of an existing user.
///
/// # Arguments
/// * `db` - Reference to the database connection wrapper.
/// * `user_id` - Telegram user ID.
/// * `city` - Name of the city to insert or update.
pub fn upsert_city(db: &Db, user_id: i64, city: &String) {
    let conn = db.lock().unwrap();
    let now = Utc::now().naive_utc();

    conn.execute(
        "INSERT INTO users (id, city, created_at, updated_at)\
        VALUES (?1, ?2, ?3, ?4)\
        ON CONFLICT(id) DO UPDATE SET city = excluded.city, updated_at = excluded.updated_at",
        params![user_id, city, now, now],
    ).unwrap();
}
