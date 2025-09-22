mod handlers;
mod db;
mod states;
mod types;
mod schema;
mod utils;
mod enums;
mod api;

use std::env;
use dotenvy::dotenv;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

use crate::db::db::init_db;
use crate::schema::schema;
use crate::states::State;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TELEGRAM_TOKEN")
        .expect("TELEGRAM_TOKEN not found in .env file");

    let bot = Bot::new(token);

    let db = init_db("users.db")
        .expect("Couldn't initialize database");

    let storage = InMemStorage::<State>::new();

    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![db.clone(), storage.clone()])
        .build()
        .dispatch()
        .await;
}
