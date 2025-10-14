use teloxide::prelude::*;

use crate::db::pool::DbPool;
use crate::enums::units::SpeedUnits;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool, unit: SpeedUnits) -> HandlerResult {
    Ok(())
}

pub async fn khp_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::KilometersPerHour).await
}

pub async fn mps_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
     handler(bot, callback, db, SpeedUnits::MetersPerSecond).await
}

pub async fn mph_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::MilesPerHour).await
}

pub async fn knots_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::Knots).await
}
