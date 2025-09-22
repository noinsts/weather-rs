use std::env;
use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::Bot;

use crate::db::db::Db;
use crate::db::queries::get_city;
use crate::types::HandlerResult;
use crate::api::{fetch_forecast, today_weather, tomorrow_weather};
use crate::api::models::{WeatherResponse, Forecast};

enum Handlers {
    Today,
    Tomorrow
}

impl Handlers {
    fn label(&self) -> &'static str {
        match self {
            Handlers::Today => "Сьогодні",
            Handlers::Tomorrow => "Завтра"
        }
    }

    fn selector(&self) -> fn(&WeatherResponse) -> Option<&Forecast> {
        match self {
            Handlers::Today => today_weather,
            Handlers::Tomorrow => tomorrow_weather
        }
    }
}

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
                .await?;
            return Ok(())
        }
    };

    let city = match get_city(&db, callback.from.id.0 as i64) {
        Some(c) => c.to_string(),
        None => {
            bot.answer_callback_query(callback.id)
                .text("Ваше рідне місто не знайдено. Спробуйте знову.")
                .await?;
            return Ok(());
        }
    };

    if let Some(message) = callback.message {
        match fetch_forecast(&city, &token).await {
            Ok(resp) => {
                if let Some(today) = selector(&resp) {
                    bot.send_message(
                        message.chat().id,
                        format!("{}: {}, {}", label, today.main.temp, today.weather[0].description)
                    )
                        .await?;
                }
                else {
                    bot.send_message(message.chat().id, "Не вдалося отримати прогноз погоди на сьогодні")
                        .await?;

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

pub async fn today_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    let today = Handlers::Today;
    weather_handler(bot, callback, today.selector(), today.label().to_string(), &db).await
}

pub async fn tomorrow_handler(bot: Bot, callback: CallbackQuery, db: Db) -> HandlerResult {
    let tomorrow = Handlers::Tomorrow;
    weather_handler(bot, callback, tomorrow.selector(), tomorrow.label().to_string(), &db).await
}
