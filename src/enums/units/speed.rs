#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SpeedUnits {
    KilometersPerHour,
    MetersPerSecond,
    MilesPerHour,
    Knots,
}

impl SpeedUnits {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpeedUnits::KilometersPerHour => "km/h",
            SpeedUnits::MetersPerSecond => "m/s",
            SpeedUnits::MilesPerHour => "mph",
            SpeedUnits::Knots => "kt",
        }
    }
    
    pub fn from_str(unit: &str) -> Option<SpeedUnits> {
        match unit { 
            "km/h" => Some(SpeedUnits::KilometersPerHour),
            "m/s" => Some(SpeedUnits::MetersPerSecond),
            "mph" => Some(SpeedUnits::MilesPerHour),
            "kt" => Some(SpeedUnits::Knots),
            &_ => None,
        }
    }
}

impl Default for SpeedUnits {
    fn default() -> Self {
        SpeedUnits::KilometersPerHour
    }
}
