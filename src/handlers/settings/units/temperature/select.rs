use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;

pub async fn celsius_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    println!("Temperature select called");
    Ok(())
}

pub async fn fahrenheit_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    println!("Temperature select called");
    Ok(())
}

pub async fn kelvin_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    println!("Temperature select called");
    Ok(())
}
