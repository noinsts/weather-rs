use std::env;
use dotenvy::dotenv;

use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TELEGRAM_TOKEN")
        .expect("TELEGRAM_TOKEN not found in .env file");

    let bot = Bot::new(token);

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .endpoint(echo)
        );

    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}

async fn echo(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        bot.send_message(msg.chat.id, text.to_string())
            .reply_to(msg.id)
            .await?;
    }
    Ok(())
}