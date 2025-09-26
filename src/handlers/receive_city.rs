use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::db::Db;
use crate::db::queries::upsert_city;
use crate::handlers::start;
use crate::types::{HandlerResult, MyDialogue};

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
pub async fn receive_city_handler(bot: Bot, dialogue: MyDialogue, msg: Message, db: Db) -> HandlerResult {
    let user_id = match msg.from {
        Some(ref user) => user.id.0 as i64,
        None => {
            return Ok(());
        }
    };

    match msg.text() {
        Some(city) if !city.trim().is_empty() => {
            upsert_city(&db, user_id, &city.to_string());
            bot.send_message(msg.chat.id, "✅ Місто збережено успішно!").await?;
            let _ = dialogue.exit().await?;
            start::message_handler(bot, msg, dialogue, db).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Будь-ласка введіть валідне місто. Спробуйте знову.").await?;
        }
    }

    Ok(())
}
