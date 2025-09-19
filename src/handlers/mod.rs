use teloxide::prelude::*;
use crate::commands::Commands;

pub mod start;

pub async fn handle_command(
    bot: Bot,
    message: Message,
    commands: Commands
) -> ResponseResult<()> {
    match commands {
        Commands::Start => start::start_handler(bot, message).await,
    }
}
