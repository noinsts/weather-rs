mod commands;
mod handlers;
mod db;
mod states;
mod types;

use std::env;
use dotenvy::dotenv;

use teloxide::prelude::*;

use commands::Commands;
use crate::db::db::init_db;
use crate::handlers::handle_command;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TELEGRAM_TOKEN")
        .expect("TELEGRAM_TOKEN not found in .env file");

    let bot = Bot::new(token);

    let db = init_db("users.db")
        .expect("Couldn't initialize database");

    let handler = Update::filter_message()
        .filter_command::<Commands>()
        .endpoint(handle_command);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![db.clone()])
        .build()
        .dispatch()
        .await;
}
