use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, MessageId};
use teloxide::types::{CallbackQuery, ParseMode};

use crate::db::db::Db;
use crate::db::queries::{get_city, user_exists};
use crate::states::State;
use crate::traits::chat::ChatSource;
use crate::types::{HandlerResult, MyDialogue};
use crate::utils::keyboard::get_hub_keyboard;

/// Universal handler for both messages and callback queries.
///
/// - If the user exists in the database -> displays the saved city.
/// - If the user does not exist -> asks the user to enter their city and updates the dialogue state.
///
/// # Arguments
///
/// - `bot` - The bot instance.
/// - `source` - The update source (message or callback query).
/// - `dialogue` - Dialogue state manager.
/// - `db` - Shared database connection.
///
/// # Returns
///
/// - [`HandlerResult`] - Ok(()) if handled successfully, otherwise an error.
async fn handler<T>(
        bot: Bot,
        source: T,
        dialogue: MyDialogue,
        db: Db
) -> HandlerResult
where
    T: ChatSource
{
    let user_id = source.user_id();
    let chat_id = ChatId(source.chat_id());

    if user_exists(&db, user_id) {
        let city = get_city(&db, user_id)
            .unwrap_or_else(|| "невідоме".to_string());

        let text = format!(
            "👋 <b>Привіт!</b>\n\n\
            🏙️ <b>Ваше місто:</b> {city}\n\n\
            🔹 Оберіть дію нижче ⬇️",
            city=city
        );

        let keyboard = get_hub_keyboard();
        send_or_edit(&bot, &source, chat_id, &text.to_string(), Some(keyboard)).await?;
    }
    else {
        let text = "👋🏻 Привіт!\n\n\
        Щоб дізнатись прогноз погоди, введіть назву вашого міста";

        send_or_edit(&bot, &source, chat_id, text, None).await?;
        dialogue.update(State::ReceiveCity).await?;
    }
    Ok(())
}

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
async fn send_or_edit<T>(
    bot: &Bot,
    source: &T,
    chat_id: ChatId,
    text: &str,
    keyboard: Option<InlineKeyboardMarkup>,
) -> HandlerResult
where
    T: ChatSource
{
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

/// Handles /start message from users.
pub async fn message_handler(bot: Bot, msg: Message, dialogue: MyDialogue, db: Db) -> HandlerResult {
    handler(bot, msg, dialogue, db).await
}

/// Handles "Start" button callbacks from inline keyboards.
pub async fn callback_handler(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue, db: Db) -> HandlerResult {
    handler(bot, callback, dialogue, db).await
}
