use teloxide::prelude::*;
use crate::db::db::Db;
use crate::db::queries::user_exists;

/// Handler of /start command
pub async fn start_handler(bot: Bot, msg: Message, db: Db) -> ResponseResult<()> {
    if let Some(user) = msg.from {
        let text = if user_exists(&db, user.id.0 as i64) {
            "Registered"
        }
        else {
            "No"
        };

        bot.send_message(msg.chat.id, text).await?;
    }
    Ok(())
}