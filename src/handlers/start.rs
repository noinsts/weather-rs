use fluent_bundle::FluentArgs;
use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::states::State;
use crate::traits::chat::ChatSource;
use crate::types::{HandlerResult, MyDialogue};
use crate::utils::keyboard::get_hub_keyboard;
use crate::utils::chat::send_or_edit;
use crate::utils::locales::get_text;

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

    let user = UserQueries::get_user(&db, user_id).await;

    match user {
        Some(user) => {
            let mut args = FluentArgs::new();
            args.set("city", user.city);
            let lang = Languages::from_str(&user.language)
                .unwrap_or(Languages::default());
            let text = get_text(lang, "hub-message", Some(&args));

            let keyboard = get_hub_keyboard(
                Languages::from_str(&user.language.to_string()).unwrap()
            );
            send_or_edit(&bot, &source, chat_id, &text, Some(keyboard)).await?;
        }
        None => {
            let text = get_text(Languages::default(), "start", None);
            send_or_edit(&bot, &source, chat_id, &text, None).await?;
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
