use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::enums::units::TemperatureUnits;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool, unit: TemperatureUnits) -> HandlerResult {
    println!("Temperature select handler called");
    Ok(())
}

pub async fn celsius_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Celsius).await
}

pub async fn fahrenheit_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Fahrenheit).await
}

pub async fn kelvin_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Kelvin).await
}
