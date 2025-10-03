use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::states::State;
use crate::traits::chat::ChatSource;
use crate::types::{HandlerResult, MyDialogue};
use crate::utils::keyboard::get_hub_keyboard;
use crate::utils::chat::send_or_edit;

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
        db: DbPool
) -> HandlerResult
where
    T: ChatSource
{
    let user_id = source.user_id();
    let chat_id = ChatId(source.chat_id());

    let user_city = UserQueries::get_city(&db, user_id).await;

    match user_city {
        Some(city) => {
            let text = format!(
                "👋 <b>Привіт!</b>\n\n\
            🏙️ <b>Ваше місто:</b> {city}\n\n\
            🔹 Оберіть дію нижче ⬇️",
                city=city
            );

            let keyboard = get_hub_keyboard();
            send_or_edit(&bot, &source, chat_id, &text, Some(keyboard)).await?;
        }
        None => {
            let text = "👋🏻 Привіт!\n\n\
            Щоб дізнатись прогноз погоди, введіть назву вашого міста";

            send_or_edit(&bot, &source, chat_id, text, None).await?;
            dialogue.update(State::ReceiveCity).await?;
        }
    }

    Ok(())
}

/// Handles /start message from users.
pub async fn message_handler(bot: Bot, msg: Message, dialogue: MyDialogue, db: DbPool) -> HandlerResult {
    handler(bot, msg, dialogue, db).await
}

/// Handles "Start" button callbacks from inline keyboards.
pub async fn callback_handler(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue, db: DbPool) -> HandlerResult {
    handler(bot, callback, dialogue, db).await
}
