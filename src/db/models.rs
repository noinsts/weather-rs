use rusqlite::{Row, Result};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub city: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            city: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    }

    pub fn new(id: i64, city: String, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> Self {
        let now = Utc::now().naive_utc();
        User {
            id,
            city,
            created_at: now,
            updated_at: now,
        }
    }
}