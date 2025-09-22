mod api;
pub mod models;
mod service;

pub use api::fetch_forecast;
pub use service::{today_weather, tomorrow_weather};
