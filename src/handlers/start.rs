use teloxide::prelude::*;

use crate::db::db::Db;
use crate::db::queries::{get_city, user_exists};
use crate::states::State;
use crate::types::{HandlerResult, MyDialogue};
use crate::utils::keyboard::get_hub_keyboard;

/// Handler of /start command
pub async fn start_handler(bot: Bot, msg: Message, dialogue: MyDialogue, db: Db) -> HandlerResult {
    if let Some(user) = msg.from {
        if user_exists(&db, user.id.0 as i64) {
            let city = get_city(&db, user.id.0 as i64)
                .unwrap_or_else(|| "unknown".to_string());

            bot.send_message(msg.chat.id, format!("Hello! Your city is {}!", city))
                .reply_markup(get_hub_keyboard())
                .await?;
        }
        else {
            bot.send_message(msg.chat.id, "Hello, please, enter ur hometown.")
                .await?;
            dialogue.update(State::ReceiveCity)
                .await?;
        }
    }
    Ok(())
}