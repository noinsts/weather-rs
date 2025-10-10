use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;
use crate::utils::chat::hub_handler;
use crate::utils::keyboard::get_settings_hub;

pub async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    hub_handler(
        &bot,
        &callback,
        &db,
        "settings-hub",
        get_settings_hub
    ).await
}