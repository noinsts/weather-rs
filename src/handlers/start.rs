use teloxide::prelude::*;
use teloxide::types::MessageId;
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

        if source.is_any().is::<CallbackQuery>() {
            if let Some(message_id) = source.message_id() {
                bot.edit_message_text(chat_id, MessageId(message_id), text)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
            }
        }
        else {
            bot.send_message(chat_id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
    }
    else {
        let text = "👋🏻 Привіт!\n\n\
        Щоб дізнатись прогноз погоди, введіть назву вашого міста";

        if source.is_any().is::<CallbackQuery>() {
            if let Some(message_id) = source.message_id() {
                bot.edit_message_text(chat_id, MessageId(message_id), text)
                    .await?;
            }
        }
        else {
            bot.send_message(chat_id, text)
                .await?;
        }
        dialogue.update(State::ReceiveCity).await?;
    }
    Ok(())
}

/// Handles /start message from users.
pub async fn message_handler(bot: Bot, msg: Message, dialogue: MyDialogue, db: Db) -> HandlerResult {
    handler(bot, msg, dialogue, db).await
}

/// Handles Start button callbacks from inline keyboards.
pub async fn callback_handler(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue, db: Db) -> HandlerResult {
    handler(bot, callback, dialogue, db).await
}
