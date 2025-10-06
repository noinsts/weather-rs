use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::Callbacks;
use crate::enums::languages::Languages;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::locales::get_text;

/// Handles a language change request.
/// Updates the user's language in the database if needed and shows a response alert.
///
/// # Arguments
/// - `bot` - Telegram bot instance.
/// - `callback` - Callback query from the user.
/// - `pool` - Database connection pool.
/// - `lang` - Target language.
///
/// # Returns
/// `HandlerResult`
async fn handler(bot: Bot, callback: CallbackQuery, pool: DbPool, lang: Languages) -> HandlerResult {
    let user_id = callback.user_id();
    let callback_id = callback.id.clone();

    let text = match UserQueries::get_user(&pool, user_id).await {
        Some(user) => {
            if Languages::from_str(&user.language.as_str()) == Some(lang) {
                get_text(lang, "language-no-change", None)
            }
            else {
                match UserQueries::set_lang(&pool, user_id, lang.as_str()).await {
                    Ok(_) => get_text(lang, "language-success", None),
                    Err(_) => get_text(lang, "error", None),
                }
            }
        }
        None => get_text(Languages::default(), "error", None),
    };

    bot.answer_callback_query(callback_id)
        .text(text)
        .show_alert(true)
        .await?;

    Ok(())
}

/// Sets the user's language to Ukrainian.
pub async fn ukrainian_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::Uk).await
}

/// Sets the user's language to English.
pub async fn english_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::En).await
}

/// Sets the user's language to Deutsch.
pub async fn deutsch_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::De).await
}