use teloxide::prelude::*;
use teloxide::types::ParseMode;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::keyboard::get_languages_keyboard;
use crate::utils::locales::get_text;

pub async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    let user = UserQueries::get_user(&db, callback.user_id()).await;

    if let Some(message) = callback.message {
        let chat_id = message.chat().id;
        let message_id = message.id();

        let lang = user
            .as_ref()
            .and_then(|u| Languages::from_str(&u.language))
            .unwrap_or_default();

        bot.edit_message_text(chat_id, message_id, get_text(lang, "settings-language-hub", None))
            .parse_mode(ParseMode::Html)
            .reply_markup(get_languages_keyboard(lang))
            .await?;
    }
    Ok(())
}