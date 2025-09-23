mod handlers;
mod db;
mod states;
mod types;
mod schema;
mod utils;
mod enums;
mod api;
mod traits;

use std::env;
use dotenvy::dotenv;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

use crate::db::db::init_db;
use crate::schema::schema;
use crate::states::State;

/// Entry point of the Telegram bot application.
///
/// Steps performed in `main`:
/// 1. Loads environment variables from `.env` file.
/// 2. Reads the Telegram bot token from `TELEGRAM_TOKEN`.
/// 3. Initializes the bot instance with `Bot::new`.
/// 4. Initializes the SQLite database `users.db`.
/// 5. Sets up in-memory dialogue storage for user states.
/// 6. Builds the `Dispatcher` with the bot, update schema, and dependencies.
/// 7. Starts polling updates and handlers Ctrl+C gracefully.
#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env variables

    // Read Telegram token
    let token = env::var("TELEGRAM_TOKEN")
        .expect("TELEGRAM_TOKEN not found in .env file");

    // Initializes the bot instance
    let bot = Bot::new(token);

    // Initialize SQLite database
    let db = init_db("users.db")
        .expect("Couldn't initialize database");

    // Create in-memory storage for user dialogue states
    let storage = InMemStorage::<State>::new();

    // Build and run the dispatcher
    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![db.clone(), storage.clone()])
        .build()
        .dispatch()
        .await;
}
