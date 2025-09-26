use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::db::Db;
use crate::db::queries::upsert_city;
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
    if let Some(user) = &msg.from {
        match msg.text() {
            Some(city) => {
                upsert_city(&db, user.id.0 as i64, &city.to_string());
                bot.send_message(msg.chat.id,"Yes.").await?;
                let _ = dialogue.exit().await;
            }
            None => {
                bot.send_message(msg.chat.id, "No city specified, try again.").await?;
            }
        }
    }
    Ok(())
}
