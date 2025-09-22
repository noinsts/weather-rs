use std::env;
use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::Bot;

use crate::db::db::Db;
use crate::db::queries::get_city;
use crate::types::HandlerResult;
use crate::api::{fetch_forecast, today_weather};
use crate::api::models::{WeatherResponse, Forecast};

async fn weather_handler<F>(
    bot: Bot,
    callback: CallbackQuery,
    selector: F,
    label: String,
    db: &Db
) -> HandlerResult
where
    F: Fn(&WeatherResponse) -> Option<&Forecast>,
{
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
    weather_handler(bot, callback, today_weather, "Сьогодні".to_string(), &db).await
}
