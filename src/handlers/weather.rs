use std::env;
use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::ParseMode;

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
async fn weather_handler<F>(
    bot: Bot,
    callback: CallbackQuery,
    selector: F,
    label: String,
    db: &Db
) -> HandlerResult
where
    F: Fn(&WeatherResponse) -> Option<&Forecast>
{
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
                if let Some(response) = selector(&resp) {
                    let desc = response.weather[0].description.clone();
                    let text = format!(
                        "🌤️ <b>Погода в {city}</b>\n\n\
                        🌡️ <b>Температура</b>: {temp}°C\n\
                        {emoji} {desc}\n\n\
                        <i>Гарного дня!</i> ☀️",
                        city=city,
                        temp=response.main.temp,
                        emoji=weather_to_emoji(&desc.to_string()),
                        desc=desc,
                    );

                    bot.edit_message_text(message.chat().id, message.id(), text)
                        .reply_markup(get_to_hub())
                        .parse_mode(ParseMode::Html)
                        .await?;
                }
                else {
                    bot.answer_callback_query(callback.id)
                        .text( format!("Не вдалося отримати прогноз погоди на {}", label.to_lowercase()))
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

/// Returns weather emoji
fn weather_to_emoji(description: &str) -> &str {
    match description.to_lowercase().as_str() {
        desc if desc.contains("дощ") => "🌧️",
        desc if desc.contains("сніг") => "❄️",
        desc if desc.contains("хмар") || desc.contains("похмуро") => "☁️",
        desc if desc.contains("ясно") || desc.contains("сонячно") => "☀️",
        desc if desc.contains("туман") => "🌫️",
        desc if desc.contains("гроза") => "⛈️",
        _ => "🌤️", // Default
    }
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
