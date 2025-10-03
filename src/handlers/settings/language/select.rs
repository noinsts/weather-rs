use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::enums::languages::Languages;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callbacks: CallbackQuery, lang: Languages) -> HandlerResult {
    print!("hello!\n");
    Ok(())
}

pub async fn ukrainian_handler(bot: Bot, callback: CallbackQuery) -> HandlerResult {
    handler(bot, callback, Languages::Uk).await
}

pub async fn english_handler(bot: Bot, callback: CallbackQuery) -> HandlerResult {
    handler(bot, callback, Languages::En).await
}