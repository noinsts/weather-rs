use chrono::{NaiveDate, Utc, Duration};
use crate::api::models::{Forecast, WeatherResponse};

fn get_weather_for_date(resp: &WeatherResponse, date: NaiveDate) -> Option<&Forecast> {
    let date_str = date.format("%Y-%m-%d").to_string();

    resp.list
        .iter()
        .find(|f| {
            f.dt_txt.starts_with(&date_str) && f.dt_txt.ends_with("12:00:00")
        })
        .or_else(|| {
            // Fallback: знаходимо будь-який запис для цього дня
            resp.list
                .iter()
                .find(|f| f.dt_txt.starts_with(&date_str))
        })
}

pub fn today_weather(response: &WeatherResponse) -> Option<&Forecast> {
    let today = Utc::now().date_naive();
    get_weather_for_date(response, today)
}

pub fn tomorrow_weather(response: &WeatherResponse) -> Option<&Forecast> {
    let tomorrow = Utc::now().date_naive() + Duration::days(1);
    get_weather_for_date(response, tomorrow)
}