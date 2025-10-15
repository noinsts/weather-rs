use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::enums::units::SpeedUnits;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::locales::get_text;

/// Handles the logic of updating user's preferred speed unit.
///
/// # Arguments
/// - `bot` - The Telegram bot instance.
/// - `callback` - The callback query triggered for user.
/// - `db` - Shared database connection pool.
/// - `unit` - The speed unit to be set.
///
/// # Returns
/// - [`HandlerResult`] - Result indication whether the handler executed successfully.
///
/// # Behavior
/// - Fetches the user from the database.
/// - If the user already has the selected temperature unit, sends "no change" message.
/// - Otherwise, update's the user temperature unit and send success message.
/// - Already replies with a callback alert.
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

/// Handles the callback to selecting **KilometersPerHour** as the speed unit.
pub async fn khp_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::KilometersPerHour).await
}

/// Handles the callback to selecting **MetersPerSecond** as the speed unit.
pub async fn mps_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
     handler(bot, callback, db, SpeedUnits::MetersPerSecond).await
}

/// Handles the callback to selecting **MilesPerHour** as the speed unit.
pub async fn mph_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::MilesPerHour).await
}

/// Handles the callback to selecting **Knots** as the speed unit.
pub async fn knots_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, SpeedUnits::Knots).await
}
