use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::db::Db;
use crate::types::{HandlerResult, MyDialogue};

pub async fn receive_city_handler(bot: Bot, dialogue: MyDialogue, msg: Message, db: Db) -> HandlerResult {
    match msg.text() {
        Some(city) => {
            bot.send_message(msg.chat.id,"Ok").await?;
            let _ = dialogue.exit().await;
        }
        None => {
            bot.send_message(msg.chat.id, "No city specified").await?;
        }
    }
    Ok(())
}
