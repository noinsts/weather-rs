use std::error::Error;
use std::sync::Arc;

use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;

/// Type alias for the database connection pool
pub type DbPool = Arc<Pool<AsyncPgConnection>>;

/// Initialize the PostgreSQL database connection pool and create table if not exists
///
/// # Arguments
/// - `database_url` - PostgreSQL connection string
///
/// # Returns
/// - `Result<DbPool, Box<dyn Error> - Thread-safe connection pool
pub async fn init_db(database_url: &str) -> Result<DbPool, Box<dyn Error>> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    let pool = Pool::builder(config)
        .max_size(10)
        .build()?;

    let mut conn = pool.get().await?;
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users (\
            id BIGINT PRIMARY KEY,\
            city TEXT NOT NULL,\
            language TEXT NOT NULL,\
            created_at TIMESTAMP NOT NULL,\
            updated_at TIMESTAMP NOT NULL\
        )"
    )
        .execute(&mut conn)
        .await?;

    Ok(Arc::new(pool))
}
