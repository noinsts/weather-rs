use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::types::HandlerResult;

pub async fn handler(bot: Bot, callback: CallbackQuery) -> HandlerResult {
    if let Some(message) = callback.message {
        let chat_id = message.chat().id;
        let message_id = message.id();

        bot.edit_message_text(chat_id, message_id, "Hello")
            .await?;
    }
    Ok(())
}