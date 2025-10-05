use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
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
pub async fn receive_city_handler(bot: Bot, dialogue: MyDialogue, msg: Message, db: DbPool) -> HandlerResult {
    let user_id = if let Some(user) = &msg.from {
        user.id.0 as i64
    }
    else {
        return Ok(());
    };

    let city = match msg.text().filter(|c| !c.trim().is_empty()) {
        Some(city) => city,
        None => {
            bot.send_message(msg.chat.id, "⚠️ Будь-ласка введіть валідне місто. Спробуйте знову.").await?;
            return Ok(());
        }
    };

    match UserQueries::upsert_city(&db, user_id, &city).await {
        Ok(_) => {
            bot.send_message(msg.chat.id, "✅ Місто збережено успішно!").await?;
            let _ = dialogue.exit().await?;
            start::message_handler(bot, msg, dialogue, db).await?;
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "❌ Помилка при збережені. Спробуйте ще раз.").await?;
        }
    }

    Ok(())
}
