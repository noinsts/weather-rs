pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Default for TemperatureUnits {
    fn default() -> Self {
        TemperatureUnits::Celsius
    }
}