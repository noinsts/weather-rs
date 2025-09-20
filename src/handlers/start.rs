use teloxide::prelude::*;

use crate::db::db::Db;
use crate::db::queries::user_exists;
use crate::states::State;
use crate::types::{HandlerResult, MyDialogue};

/// Handler of /start command
pub async fn start_handler(bot: Bot, msg: Message, dialogue: MyDialogue, db: Db) -> HandlerResult {
    if let Some(user) = msg.from {
        if user_exists(&db, user.id.0 as i64) {
            bot.send_message(msg.chat.id, "Hello!")
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