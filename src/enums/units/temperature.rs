#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnits {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnits::Celsius => "C",
            TemperatureUnits::Fahrenheit => "F",
            TemperatureUnits::Kelvin => "K",
        }
    }
}

impl Default for TemperatureUnits {
    fn default() -> Self {
        TemperatureUnits::Celsius
    }
}