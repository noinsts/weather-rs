use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::Message;

use crate::db::db::Db;
use crate::db::queries::upsert_city;
use crate::types::{HandlerResult, MyDialogue};

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
