use std::error::Error;
use chrono::Utc;

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

use super::models::UserData;
use super::pool::DbPool;
use super::schema::users;

pub struct UserQueries;

impl UserQueries {
    pub async fn exists(pool: &DbPool, user_id: i64) -> bool {
        let mut conn = match pool.get().await {
            Ok(conn) => conn,
            Err(_) => return false,
        };

        users::table
            .filter(users::id.eq(user_id))
            .select(users::id)
            .first::<i64>(&mut conn)
            .await
            .is_ok()
    }

    pub async fn get_city(pool: &DbPool, user_id: i64) -> Option<String> {
        let mut conn = pool.get().await.ok()?;

        users::table
            .filter(users::id.eq(user_id))
            .select(users::city)
            .first::<String>(&mut conn)
            .await
            .ok()
    }

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