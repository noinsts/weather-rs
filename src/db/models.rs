use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::enums::languages::Languages;
use crate::enums::units::{SpeedUnits, TemperatureUnits};
use super::schema::users;

/// Модель користувача для читання з бази даних
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub city: String,
    pub language: String,
    pub temperature_unit: String,
    pub speed_unit: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Модель для створення/оновлення користувача
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserData {
    pub id: i64,
    pub city: String,
    pub language: String,
    pub temperature_unit: String,
    pub speed_unit: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserData {
    /// Створює нові дані користувача з поточним часом
    pub fn new(id: i64, city: String, lang: Languages, temperature_unit: TemperatureUnits, speed_unit: SpeedUnits) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id,
            city,
            language: lang.as_str().to_string(),
            temperature_unit: temperature_unit.as_str().to_string(),
            speed_unit: speed_unit.as_str().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
