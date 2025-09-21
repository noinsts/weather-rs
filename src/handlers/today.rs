use teloxide::prelude::*;
use teloxide::Bot;

use crate::db::db::Db;
use crate::types::HandlerResult;

pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    if let Some(message) = callback.message {
        bot.send_message(message.chat().id, "Weather today!").await?;
    }
    bot.answer_callback_query(callback.id).await?;
    Ok(())
}
