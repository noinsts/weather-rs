use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::enums::languages::Languages;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callback: CallbackQuery, pool: DbPool, lang: Languages) -> HandlerResult {
    Ok(())
}

pub async fn ukrainian_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::Uk).await
}

pub async fn english_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::En).await
}