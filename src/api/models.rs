use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub list: Vec<Forecast>,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}
