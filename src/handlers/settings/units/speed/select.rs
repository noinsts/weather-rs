use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::enums::units::SpeedUnits;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::locales::get_text;

async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool, unit: SpeedUnits) -> HandlerResult {
    let user_id = callback.user_id();
    let callback_id = callback.id.clone();

    let text = match UserQueries::get_user(&db, user_id).await {
        Some(user) => {
            let lang = Languages::from_str(&user.language).unwrap_or_default();

            if SpeedUnits::from_str(&user.speed_unit) == Some(unit) {
                get_text(lang, "speed-unit-no-change", None)
            }
            else {
                match UserQueries::set_speed_unit(&db, user_id, unit.as_str()).await {
                    Ok(_) => get_text(lang, "speed-unit-success", None),
                    Err(_) => get_text(lang, "error", None)
                }
            }
        }
        None => get_text(Languages::default(), "error", None),
    };

    bot.answer_callback_query(callback_id)
        .text(text)
        .show_alert(true)
        .await?;

    Ok(())
}

pub async fn khp_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::KilometersPerHour).await
}

pub async fn mps_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
     handler(bot, callback, db, SpeedUnits::MetersPerSecond).await
}

pub async fn mph_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::MilesPerHour).await
}

pub async fn knots_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::Knots).await
}
