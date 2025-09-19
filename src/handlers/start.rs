use teloxide::prelude::*;

/// Handler of /start command
pub async fn start_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = if let Some(user) = &msg.from() {
        format!("👋 Hello, {}! Welcome to the weather bot!", user.first_name)
    }
    else {
        "👋 Hello! Welcome to the weather bot!".to_string()
    };

    if let Err(e) = bot.send_message(msg.chat.id, text)
        .await
    {
        eprintln!("Error sending message: {:?}", e);
    }
    Ok(())
}