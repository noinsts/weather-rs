use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, MessageId, ParseMode};

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::locales::get_text;

/// Sends or edits a message depending on the update source.
///
/// - If `source` is a [`CallbackQuery`], the existing message will be edited.
/// - Otherwise, a new message will be sent.
/// - All messages are sent with [`ParseMode::Html`] enabled, so HTML tags
///   (e.g. `<b>`, `<i>`, `<u>`) are supported in the text.
///
/// Optionally attaches an inline keyboard if provided.
///
/// # Arguments
/// * `bot` - Reference to the [`Bot`] instance used to send or edit the message.
/// * `source` - Any type implementing [`ChatSource`] (`Message` or `CallbackQuery`).
/// * `chat_id` - The target chat ID where the message will be sent or edited.
/// * `text` - The message content. Supports HTML formatting.
/// * `keyboard` - Optional [`InlineKeyboardMarkup`].
///   If `Some(kb)` → the keyboard is attached,
///   If `None` → no keyboard is included.
///
/// # Returns
/// * [`HandlerResult`] - Ok(()) if successful, or an error if the Telegram API call fails.
pub async fn send_or_edit<T>(
    bot: &Bot,
    source: &T,
    chat_id: ChatId,
    text: &str,
    keyboard: Option<InlineKeyboardMarkup>,
) -> HandlerResult
where
    T: ChatSource
{
    // Check if the update source is a CallbackQuery
    if source.is_any().is::<CallbackQuery>() {
        if let Some(message_id) = source.message_id() {
            let mut req = bot.edit_message_text(chat_id, MessageId(message_id), text)
                .parse_mode(ParseMode::Html);

            if let Some(keyboard) = keyboard {
                req = req.reply_markup(keyboard);
            }

            req.await?;
        }
    }
    else {
        let mut req = bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html);

        if let Some(keyboard) = keyboard {
            req = req.reply_markup(keyboard);
        }

        req.await?;
    }

    Ok(())
}

pub async fn hub_handler<F>(
    bot: &Bot,
    callback: &CallbackQuery,
    db: &DbPool,
    locale_key: &str,
    keyboard_fn: F,
) -> HandlerResult
where
    F: Fn(Languages) -> InlineKeyboardMarkup,
{
    let user = UserQueries::get_user(&db, callback.user_id()).await;

    if let Some(message) = &callback.message {
        let chat_id = message.chat().id;
        let message_id = message.id();

        let lang = user
            .as_ref()
            .and_then(|u| Languages::from_str(&u.language))
            .unwrap_or_default();

        bot.edit_message_text(chat_id, message_id, get_text(lang, locale_key, None))
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard_fn(lang))
            .await?;
    }
    Ok(())
}