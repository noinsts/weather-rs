use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::enums::units::TemperatureUnits;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;

async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool, unit: TemperatureUnits) -> HandlerResult {
    println!("Temperature select handler called");

    let user_id = callback.user_id();
    let callback_id = callback.id.clone();

    let text = match UserQueries::get_user(&db, user_id).await {
        Some(user) => {
            // let lang = Languages::from_str(&user.language);

            if TemperatureUnits::from_str(&user.temperature_unit.as_str()) == Some(unit) {
                "Нічого не змінилось".to_string()
            }
            else {
                match UserQueries::set_temp_unit(&db, user_id, unit.as_str()).await {
                    Ok(_) => {
                        "Все чудово!".to_string()
                    }
                    Err(_) => {
                        "Помилка".to_string()
                    }
                }
            }

            // something
        }
        None => {
            "Помилка".to_string()
        }
    };

    bot.answer_callback_query(callback_id)
        .text(text)
        .show_alert(true)
        .await?;

    Ok(())
}

pub async fn celsius_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Celsius).await
}

pub async fn fahrenheit_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Fahrenheit).await
}

pub async fn kelvin_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Kelvin).await
}
