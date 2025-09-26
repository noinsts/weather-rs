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
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: i64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}


#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}