use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::db::pool::DbPool;
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::enums::units::TemperatureUnits;
use crate::traits::chat::ChatSource;
use crate::types::HandlerResult;
use crate::utils::locales::get_text;

/// Handles the logic of updating a user's preferred temperature unit.
///
/// # Arguments
/// - `bot` - The Telegram bot instance.
/// - `callback` - The callback query triggered by the user.
/// - `db` - The database connection pool.
/// - `unit` - The temperature unit to be set.
///
/// # Returns
/// - [`HandlerResult`] - Result indication whether the handler executed successfully.
///
/// # Behavior
/// - Fetches the user from the database.
/// - If the user already has the selected temperature unit, sends a "no change" message.
/// - Otherwise, updates the user's temperature unit and sends a success message.
/// - Always replies with a callback alert.
async fn handler(bot: Bot, callback: CallbackQuery, db: DbPool, unit: TemperatureUnits) -> HandlerResult {
    let user_id = callback.user_id();
    let callback_id = callback.id.clone();

    let text = match UserQueries::get_user(&db, user_id).await {
        Some(user) => {
            let lang = Languages::from_str(&user.language).unwrap_or_default();

            if TemperatureUnits::from_str(&user.temperature_unit.as_str()) == Some(unit) {
                get_text(lang, "temperature-unit-no-change", None)
            }
            else {
                match UserQueries::set_temp_unit(&db, user_id, unit.as_str()).await {
                    Ok(_) => get_text(lang, "temperature-unit-success", None),
                    Err(_) => get_text(lang, "error", None)
                }
            }
        }
        None => get_text(Languages::default(), "error", None)
    };

    bot.answer_callback_query(callback_id)
        .text(text)
        .show_alert(true)
        .await?;

    Ok(())
}

/// Handles the callback for selecting **Celsius** as the temperature unit.
pub async fn celsius_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Celsius).await
}

/// Handles the callback for selecting **Fahrenheit** as the temperature unit.
pub async fn fahrenheit_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Fahrenheit).await
}

/// Handles the callback for selecting **Kelvin** as the temperature unit.
pub async fn kelvin_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    handler(bot, callback, db, TemperatureUnits::Kelvin).await
}
