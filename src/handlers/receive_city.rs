use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::handlers::start;
use crate::types::{HandlerResult, MyDialogue};
use crate::utils::locales::get_text;

/// Handler receiving the user's city.
///
/// Expects the user to send a text message containing their city name.
/// Saves the city to the database and exits the dialogue
///
/// # Arguments
/// * `bot` - The Telegram bot instance.
/// * `dialogue` - Dialogue state for the user.
/// * `msg` - Incoming message containing the city.
/// * `db` - Database connection wrapper.
pub async fn receive_city_handler(bot: Bot, dialogue: MyDialogue, msg: Message, db: DbPool) -> HandlerResult {
    let user_id = if let Some(user) = &msg.from {
        user.id.0 as i64
    }
    else {
        return Ok(());
    };

    let user = UserQueries::get_user(&db, user_id).await;

    let lang = user
        .as_ref()
        .and_then(|u| Languages::from_str(&u.language))
        .unwrap_or_default();

    let city = match msg.text().filter(|c| !c.trim().is_empty()) {
        Some(city) => city,
        None => {
            bot.send_message(msg.chat.id, get_text(lang, "validation-city", None)).await?;
            return Ok(());
        }
    };

    match UserQueries::upsert_city(&db, user_id, &city).await {
        Ok(_) => {
            bot.send_message(msg.chat.id, get_text(lang, "save-city-success", None)).await?;
            let _ = dialogue.exit().await?;
            start::message_handler(bot, msg, dialogue, db).await?;
        }
        Err(_) => {
            bot.send_message(msg.chat.id, get_text(lang, "saving-error", None)).await?;
        }
    }

    Ok(())
}
