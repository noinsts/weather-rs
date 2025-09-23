use std::env;
use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::Bot;

use crate::db::db::Db;
use crate::db::queries::get_city;
use crate::types::HandlerResult;
use crate::api::{fetch_forecast, today_weather, tomorrow_weather};
use crate::api::models::{WeatherResponse, Forecast};
use crate::utils::keyboard::get_to_hub;

/// Weather handler type, representing available forecast options.
enum Handlers {
    /// Forecast for today.
    Today,
    /// Forecast for tomorrow.
    Tomorrow
}

impl Handlers {
    /// Returns localized label for the forecast option.
    fn label(&self) -> &'static str {
        match self {
            Handlers::Today => "Сьогодні",
            Handlers::Tomorrow => "Завтра"
        }
    }

    /// Returns a selector function that extracts the right forecast
    fn selector(&self) -> fn(&WeatherResponse) -> Option<&Forecast> {
        match self {
            Handlers::Today => today_weather,
            Handlers::Tomorrow => tomorrow_weather
        }
    }
}

/// Generic weather handler used by both `today_handler` and `tomorrow_handler`.
///
/// Steps:
///     - Reads the `WEATHER_API_KEY` from environment.
///     - Fetcher user's city from the database.
///     - Calls the weather API and extracts forecast using provided selector.
///     - Edits the callback message with forecast result and attach "back to hub" keyboard.
///     - In case of errors (missing API key, no city, API error), responds to the callback query with an error message.
async fn weather_handler(
    bot: Bot,
    callback: CallbackQuery,
    selector: fn(&WeatherResponse) -> Option<&Forecast>,
    label: String,
    db: &Db
) -> HandlerResult {
    dotenv().ok();

    let token = match env::var("WEATHER_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("WEATHER_API_KEY environment variable not set");
            bot.answer_callback_query(callback.id)
                .text("Помилка, зверніться до розробників.")
                .show_alert(true)
                .await?;
            return Ok(())
        }
    };

    let city = match get_city(&db, callback.from.id.0 as i64) {
        Some(c) => c.to_string(),
        None => {
            bot.answer_callback_query(callback.id)
                .text("Ваше рідне місто не знайдено. Спробуйте знову.")
                .show_alert(true)
                .await?;
            return Ok(());
        }
    };

    if let Some(message) = callback.message {
        match fetch_forecast(&city, &token).await {
            Ok(resp) => {
                if let Some(today) = selector(&resp) {
                    bot.edit_message_text(
                        message.chat().id,
                        message.id(),
                        format!("{}: {}, {}", label, today.main.temp, today.weather[0].description)
                    )
                        .reply_markup(get_to_hub())
                        .await?;
                }
                else {
                    bot.answer_callback_query(callback.id)
                        .text( "Не вдалося отримати прогноз погоди на сьогодні")
                        .show_alert(true)
                        .await?;
                    return Ok(());
                }
            }
            Err(_) => {
                bot.send_message(message.chat().id, "Помилка при отриманні даних про погоду.")
                    .await?;
            }
        }
    }
    bot.answer_callback_query(callback.id).await?;
    Ok(())
}

/// Handler for today weather.
pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    let today = Handlers::Today;
    weather_handler(bot, callback, today.selector(), today.label().to_string(), &db).await
}

/// Handler for tomorrow weather.
pub async fn tomorrow_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    let tomorrow = Handlers::Tomorrow;
    weather_handler(bot, callback, tomorrow.selector(), tomorrow.label().to_string(), &db).await
}
