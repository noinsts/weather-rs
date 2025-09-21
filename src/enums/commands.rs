use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
pub enum Commands {
    #[command(description = "Start the bot and show welcome prompt")]
    Start,
}
