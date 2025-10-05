use std::env;
use dotenvy::dotenv;
use fluent_bundle::FluentArgs;
use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::ParseMode;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;
use crate::api::{fetch_forecast, today_weather, tomorrow_weather};
use crate::api::models::{WeatherResponse, Forecast};
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::fluent_args;
use crate::utils::keyboard::get_to_hub;
use crate::utils::locales::get_text;
use crate::utils::string::capitalize_first_letter;

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
    db: &DbPool
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
    db: &DbPool,
) -> Result<(), WeatherError> {
    let config = WeatherConfig::from_env()?;
    let user_id = callback.from.id.0 as i64;

    let user = UserQueries::get_user(db, user_id)
        .await
        .ok_or(WeatherError::CityNotFound)?;

    let lang = Languages::from_str(&user.language).unwrap_or_default();

    let message = callback.message
        .as_ref()
        .ok_or(WeatherError::MissingMessage)?;

    let weather_response = fetch_forecast(&user.city, &config.api_key, lang)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    let forecast = period.selector()(&weather_response)
        .ok_or(WeatherError::NoForecastData)?;

    let formatted_message = format_weather_message(&user.city, period, &forecast, lang);

    bot.edit_message_text(message.chat().id, message.id(), formatted_message)
        .reply_markup(get_to_hub(lang))
        .parse_mode(ParseMode::Html)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    Ok(())
}

/// Formats weather information into a user-friendly message
fn format_weather_message(city: &str, period: WeatherPeriod, response: &Forecast, lang: Languages) -> String {
    let description = &response.weather[0].description;
    let emoji = weather_to_emoji(description);
    let wind_speed = if response.wind.speed as i32 == 0 {
        "відсутній".to_string()
    }
    else {
        format!("{} км/год", response.wind.speed as i32)
    };

    let args = fluent_args![
        "city" => city,
        "day" => period.label().to_lowercase(),
        "emoji" => emoji,
        "description" => capitalize_first_letter(description),
        "temp" => response.main.temp as i32,
        "feels_like" => response.main.feels_like as i32,
        "humidity" => response.main.humidity,
        "wind_speed" => wind_speed,
    ];

    get_text(lang, "weather", Some(&args))
}

/// Returns weather emoji
fn weather_to_emoji(description: &str) -> &'static str {
    match description.to_lowercase().as_str() {
        desc if desc.contains("дощ") || desc.contains("rain") => "🌧️",
        desc if desc.contains("сніг") || desc.contains("snow") => "❄️",
        desc if desc.contains("хмар") || desc.contains("похмуро") || desc.contains("cloud") || desc.contains("cloudy") => "☁️",
        desc if desc.contains("ясно") || desc.contains("сонячно") || desc.contains("sun") || desc.contains("sunny") => "☀️",
        desc if desc.contains("туман") || desc.contains("fog") || desc.contains("mist") => "🌫️",
        desc if desc.contains("гроза") || desc.contains("thunder") || desc.contains("storm") => "⛈️",
        _ => "🌤️", // Default
    }
}

/// Handler for today weather.
pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Today, &db).await
}

/// Handler for tomorrow weather.
pub async fn tomorrow_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Tomorrow, &db).await
}
