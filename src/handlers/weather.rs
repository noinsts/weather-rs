use std::env;
use std::fmt::format;
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
#[derive(Debug, Clone, Copy)]
enum WeatherPeriod {
    /// Forecast for today.
    Today,
    /// Forecast for tomorrow.
    Tomorrow
}

impl WeatherPeriod {
    /// Returns localized label for the forecast option.
    const fn label(&self) -> &'static str {
        match self {
            WeatherPeriod::Today => "Сьогодні",
            WeatherPeriod::Tomorrow => "Завтра"
        }
    }

    /// Returns a selector function that extracts the right forecast
    const fn selector(&self) -> fn(&WeatherResponse) -> Option<&Forecast> {
        match self {
            WeatherPeriod::Today => today_weather,
            WeatherPeriod::Tomorrow => tomorrow_weather
        }
    }
}

/// Errors that can occur during weather handler
#[derive(Debug)]
enum WeatherError {
    /// API key not found in environment
    MissingApiKey,

    /// User's city not found in database
    CityNotFound,

    /// Failed to fetch weather data
    ApiFetchError,

    /// No forecast data available for requested period
    NoForecastData,

    /// Missing message in callback query
    MissingMessage
}

impl WeatherError {
    /// Returns user-friendly error message
    const fn user_message(&self) -> &'static str {
        match self {
            WeatherError::MissingApiKey => "Помилка сервісу, зверніться до розробників",
            WeatherError::CityNotFound => "Ваше місто не знайдено. Спробуйте встановити його знову.",
            WeatherError::ApiFetchError => "Не вдалося отримати дані про погоду. Спробуйте пізніше",
            WeatherError::NoForecastData => "Прогноз погоди недоступний для обраного періоду",
            WeatherError::MissingMessage => "Помилка обробки запиту.",
        }
    }
}


/// Configuration for weather service
struct WeatherConfig {
    api_key: String,
}

impl WeatherConfig {
    /// Creates new weather configuration by reading from environment
    fn from_env() -> Result<Self, WeatherError> {
        dotenv().ok();

        let api_key = env::var("WEATHER_API_KEY")
            .map_err(|_| {
                eprintln!("WEATHER_API_KEY environment variable not set");
                WeatherError::MissingApiKey
            })?;

        Ok(Self { api_key })
    }
}

/// Generic weather handler used by both `today_handler` and `tomorrow_handler`.
///
/// 1. Reads the `WEATHER_API_KEY` from environment.
/// 2. Fetcher user's city from the database.
/// 3. Calls the weather API and extracts forecast using provided selector.
/// 4. Edits the callback message with forecast result and attach "back to hub" keyboard.
/// 5. In case of errors (missing API key, no city, API error), responds to the callback query with an error message.
async fn weather_handler(
    bot: Bot,
    callback: CallbackQuery,
    period: WeatherPeriod,
    db: &Db
) -> HandlerResult {
    let callback_id = callback.id.clone();

    match handle_weather_request(&bot, &callback, period, db).await {
        Ok(_) => {
            bot.answer_callback_query(callback_id).await?;
        }
        Err(e) => {
            bot.answer_callback_query(callback_id)
                .text(e.user_message())
                .show_alert(true)
                .await?;
        }
    }

    Ok(())
}

/// Internal handler that processes weather request and returns structured errors
async fn handle_weather_request(
    bot: &Bot,
    callback: &CallbackQuery,
    period: WeatherPeriod,
    db: &Db,
) -> Result<(), WeatherError> {
    let config = WeatherConfig::from_env()?;
    let user_id = callback.from.id.0 as i64;

    let city = get_city(db, user_id)
        .ok_or(WeatherError::CityNotFound)?;

    let message = callback.message
        .as_ref()
        .ok_or(WeatherError::MissingMessage)?;

    let weather_response = fetch_forecast(&city, &config.api_key)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    let forecast = (period.selector())(&weather_response)
        .ok_or(WeatherError::NoForecastData)?;

    let formatted_message = format_weather_message(&city, period, &forecast);

    bot.edit_message_text(message.chat().id, message.id(), formatted_message)
        .reply_markup(get_to_hub())
        .parse_mode(ParseMode::Html)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    Ok(())
}

/// Formats weather information into a user-friendly message
fn format_weather_message(city: &str, period: WeatherPeriod, response: &Forecast) -> String {
    let description = &response.weather[0].description;
    let emoji = weather_to_emoji(description);
    let wind_speed = if response.wind.speed as i32 == 0 {
        "відсутній".to_string()
    }
    else {
        format!("{} км/год", response.wind.speed as i32)
    };

    format!(
        "🌤️ <b>Погода в {city} на {day}</b>\n\n\
        {emoji} {description}\n\n\
        🌡️ <b>Температура</b>: {temp}°C (відчувається як {feels_like}°C)\n\
        💧 <b>Вологість</b>: {humidity}%\n\
        💨 <b>Вітер</b>: {wind_speed}\n\n\
        <i>Гарного дня!</i> ☀️",
        city=city,
        day=period.label().to_lowercase(),
        temp=response.main.temp as i32,
        feels_like=response.main.feels_like as i32,
        humidity=response.main.humidity,
        wind_speed=wind_speed,
        emoji=emoji,
        description=capitalize_first_letter(description),
    )
}

/// Returns weather emoji
fn weather_to_emoji(description: &str) -> &'static str {
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

/// Function for capitalizing the first letter of string
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Handler for today weather.
pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Today, &db).await
}

/// Handler for tomorrow weather.
pub async fn tomorrow_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Tomorrow, &db).await
}
