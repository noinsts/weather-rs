use crate::api::models::WeatherResponse;

pub async fn fetch_forecast(city: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric&lang=ua",
        city, api_key
    );

    let resp = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(resp)
}
