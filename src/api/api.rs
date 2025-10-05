use crate::api::models::WeatherResponse;
use crate::enums::languages::Languages;

pub async fn fetch_forecast(city: &str, api_key: &str, lang: Languages) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric&lang={}",
        city, api_key, lang.as_str()
    );

    let resp = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(resp)
}
