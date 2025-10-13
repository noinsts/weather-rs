use std::env;
use dotenvy::dotenv;
use fluent_bundle::FluentArgs;
use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::ParseMode;
use once_cell::sync::Lazy;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;
use crate::api::{fetch_forecast, today_weather, tomorrow_weather};
use crate::api::models::{WeatherResponse, Forecast};
use crate::db::queries::UserQueries;
use crate::enums::languages::Languages;
use crate::enums::units::TemperatureUnits;
use crate::fluent_args;
use crate::traits::chat::ChatSource;
use crate::utils::keyboard::get_to_hub;
use crate::utils::locales::get_text;
use crate::utils::string::capitalize_first_letter;

static WEATHER_CONFIG: Lazy<Result<String, String>> = Lazy::new(|| {
    dotenv().ok();
    env::var("WEATHER_API_KEY").map_err(|_| {
        eprintln!("WEATHER_API_KEY environment variable not set");
        String::from("Missing API key")
    })
});

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
    fn label(&self, lang: Languages) -> String {
        let key = match self {
            WeatherPeriod::Today => "today",
            WeatherPeriod::Tomorrow => "tomorrow",
        };
        get_text(lang, key, None)
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

    /// User not found in database
    UserNotFound,

    /// Failed to fetch weather data
    ApiFetchError,

    /// No forecast data available for requested period
    NoForecastData,

    /// Missing message in callback query
    MissingMessage
}

impl WeatherError {
    /// Returns user-friendly error message
    fn user_message(&self, lang: Languages) -> String {
        let key = match self {
            WeatherError::MissingApiKey => "service-error",
            WeatherError::UserNotFound => "user-not-found",
            WeatherError::ApiFetchError => "api-fetch-error",
            WeatherError::NoForecastData => "no-forecast-data",
            WeatherError::MissingMessage => "missing-message",
        };
        get_text(lang, key, None)
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

    let user = UserQueries::get_user(db, callback.user_id())
        .await
        .ok_or_else(|| WeatherError::UserNotFound)
        .and_then(|u| {
            Languages::from_str(&u.language)
                .ok_or(WeatherError::UserNotFound)
                .map(|lang| (u, lang))
        });

    match user {
        Ok((user, lang)) => {
            match handle_weather_request(
                &bot,
                &callback,
                period,
                user.city,
                lang,
                TemperatureUnits::from_str(&user.temperature_unit).unwrap_or_default()
            )
                .await
            {
                Ok(_) => {
                    bot.answer_callback_query(callback_id).await?;
                }
                Err(e) => {
                    bot.answer_callback_query(callback_id)
                        .text(e.user_message(lang))
                        .show_alert(true)
                        .await?;
                }
            }
        }
        Err(e) => {
            bot.answer_callback_query(callback_id)
                .text(e.user_message(Languages::default()))
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
    city: String,
    lang: Languages,
    temperature_unit: TemperatureUnits
) -> Result<(), WeatherError> {
    let api_key = WEATHER_CONFIG
        .as_ref()
        .map_err(|_| WeatherError::MissingApiKey)?;

    let message = callback.message
        .as_ref()
        .ok_or(WeatherError::MissingMessage)?;

    let weather_response = fetch_forecast(&city, api_key, lang)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    let forecast = period
        .selector()(&weather_response)
        .ok_or(WeatherError::NoForecastData)?;

    let formatted_message = format_weather_message(&city, period, &forecast, lang, temperature_unit);

    bot.edit_message_text(message.chat().id, message.id(), formatted_message)
        .reply_markup(get_to_hub(lang))
        .parse_mode(ParseMode::Html)
        .await
        .map_err(|_| WeatherError::ApiFetchError)?;

    Ok(())
}

/// Formats weather information into a user-friendly message
fn format_weather_message(
    city: &str,
    period: WeatherPeriod,
    response: &Forecast,
    lang: Languages,
    temperature_unit: TemperatureUnits
) -> String {
    let description = &response.weather[0].description;
    let emoji = weather_to_emoji(description);
    let temp = convert_temperature(response.main.temp, temperature_unit);
    let feels_like = convert_temperature(response.main.feels_like, temperature_unit);
    let wind_speed = if response.wind.speed as i32 == 0 {
        get_text(lang, "weather-wind-speed-unknown", None)
    }
    else {
        format!("{} {}", response.wind.speed as i32, get_text(lang, "weather-wind-speed-kmh", None))
    };

    let args = fluent_args![
        "city" => city,
        "day" => period.label(lang).to_lowercase(),
        "emoji" => emoji,
        "description" => capitalize_first_letter(description),
        "temp" => temp as i32,
        "feels_like" => feels_like as i32,
        "humidity" => response.main.humidity,
        "wind_speed" => wind_speed,
        "temp_unit" => temperature_unit.as_str(),
    ];

    get_text(lang, "weather", Some(&args))
}

fn convert_temperature(celsius: f64, unit: TemperatureUnits) -> f64 {
    match unit {
        TemperatureUnits::Celsius => celsius,
        TemperatureUnits::Fahrenheit => (celsius * 9.0 / 5.0) + 32.0,
        TemperatureUnits::Kelvin => celsius + 273.15,
    }
}

const WEATHER_EMOJI_PATTERNS: &[(&[&str], &str)] = &[
    (&["дощ", "rain", "regen"], "🌧️"),
    (&["сніг", "snow", "schnee"], "❄️"),
    (&["хмар", "похмуро", "cloud", "cloudy", "wolken", "bewölkt"], "☁️"),
    (&["ясно", "сонячно", "sun", "sunny", "klar"], "☀️"),
    (&["туман", "fog", "mist", "nebel"], "🌫️"),
    (&["гроза", "thunder", "storm", "gewitter"], "⛈️"),
];

/// Returns weather emoji
fn weather_to_emoji(description: &str) -> &'static str {
    WEATHER_EMOJI_PATTERNS
        .iter()
        .find(|(patterns, _)| {
            patterns.iter().any(|pattern| description.to_lowercase().contains(pattern))
        })
        .map(|(_, emoji)| *emoji)
        .unwrap_or("🌤️")
}

/// Handler for today weather.
pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Today, &db).await
}

/// Handler for tomorrow weather.
pub async fn tomorrow_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult {
    weather_handler(bot, callback, WeatherPeriod::Tomorrow, &db).await
}
