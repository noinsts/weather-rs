use std::error::Error;
use chrono::Utc;

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

use crate::enums::languages::Languages;
use super::models::{User, UserData};
use super::pool::DbPool;
use super::schema::users;

/// Provides database query methods related to user data.
pub struct UserQueries;

impl UserQueries {
    /// Returns the full user record by ID.
    ///
    /// # Arguments
    /// - `pool` - the database connection pool
    /// - `user_id` - ID of the user
    ///
    /// # Returns
    /// - `Some(UserData)` if found
    /// - `None` if user does not exist or query fails
    pub async fn get_user(pool: &DbPool, user_id: i64) -> Option<User> {
        let mut conn = pool.get().await.ok()?;

        users::table
            .filter(users::id.eq(user_id))
            .first::<User>(&mut conn)
            .await
            .ok()
    }

    /// Inserts of updates a user`s city in the database
    ///
    /// If the user does not exist, a new record is created.
    /// If the user exists, their `city` and `updated_at` fields are updated.
    ///
    /// # Arguments
    /// - `pool` - the database connection pool
    /// - `user_id` - ID of the user
    /// - `city` - the city to insert or update
    ///
    /// # Returns
    /// - `Ok(())` on success, or an error if the operation fails.
    pub async fn upsert_city(
        pool: &DbPool,
        user_id: i64,
        city: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut conn = pool.get().await?;
        let now = Utc::now().naive_utc();

        diesel::insert_into(users::table)
            .values(&UserData {
                id: user_id,
                city: city.to_string(),
                language: Languages::default().as_str().to_string(),
                created_at: now,
                updated_at: now,
            })
            .on_conflict(users::id)
            .do_update()
            .set((
                users::city.eq(city),
                users::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}