use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callback: CallbackQuery, pool: DbPool, lang: Languages) -> HandlerResult {
    let user_id = callback.user_id();

    let user = UserQueries::get_user(&pool, user_id).await;

    match user {
        Some(user) => {
            if Languages::from_str(&user.language.as_str()) == Some(lang) {
                bot.answer_callback_query(callback.id)
                    .text("Nooo")
                    .show_alert(true)
                    .await?;
            }
            else {
                let resp = UserQueries::set_lang(&pool, user_id, lang.as_str()).await;
                match resp {
                    Ok(_) => {
                        bot.answer_callback_query(callback.id)
                            .text("Success")
                            .await?;
                    }
                    Err(e) => {
                        eprintln!("Error while update language: {:?}", e);
                        bot.answer_callback_query(callback.id)
                            .text("Error")
                            .show_alert(true)
                            .await?;
                    }
                }
            }
        }
        None => {
            bot.answer_callback_query(callback.id)
                .text("Error")
                .show_alert(true)
                .await?;
        }
    }

    Ok(())
}

pub async fn ukrainian_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::Uk).await
}

pub async fn english_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, Languages::En).await
}