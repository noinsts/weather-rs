use teloxide::prelude::*;
use crate::commands::Commands;
use crate::db::db::Db;

pub mod start;

pub async fn handle_command(
    bot: Bot,
    message: Message,
    commands: Commands,
    db: Db
) -> ResponseResult<()> {
    match commands {
        Commands::Start => start::start_handler(bot, message, db).await,
    }
}
