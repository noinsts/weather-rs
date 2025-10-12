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

    pub fn from_str(unit: &str) -> Option<Self> {
        match unit {
            "C" => Some(TemperatureUnits::Celsius),
            "F" => Some(TemperatureUnits::Fahrenheit),
            "K" => Some(TemperatureUnits::Kelvin),
            &_ => None
        }
    }
}

impl Default for TemperatureUnits {
    fn default() -> Self {
        TemperatureUnits::Celsius
    }
}