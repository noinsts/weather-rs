use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;
use crate::utils::chat::hub_handler;
use crate::utils::keyboard::get_languages_keyboard;

pub async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    hub_handler(
        &bot,
        &callback,
        &db,
        "settings-language-hub",
        get_languages_keyboard
    ).await
}