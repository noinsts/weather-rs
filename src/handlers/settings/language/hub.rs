use teloxide::prelude::*;

use crate::types::HandlerResult;
use crate::utils::keyboard::get_languages_keyboard;

pub async fn handler(bot: Bot, callback: CallbackQuery) -> HandlerResult {
    if let Some(message) = callback.message {
        let chat_id = message.chat().id;
        let message_id = message.id();

        bot.edit_message_text(chat_id, message_id, "Select language")
            .reply_markup(get_languages_keyboard())
            .await?;
    }
    Ok(())
}