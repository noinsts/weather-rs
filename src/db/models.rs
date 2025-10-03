use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::schema::users;

/// Модель користувача для читання з бази даних
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub city: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Модель для створення/оновлення користувача
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserData {
    pub id: i64,
    pub city: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserData {
    /// Створює нові дані користувача з поточним часом
    pub fn new(id: i64, city: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id,
            city,
            created_at: now,
            updated_at: now,
        }
    }
}