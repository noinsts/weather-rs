pub enum Callbacks {
    Start,
    Today,
    Tomorrow,

    // Settings
    SettingsHub,
    SelectLanguage,
    SelectUnits,
    
    // Languages
    English,
    Ukrainian,
    Deutsch,

    // Units
    Temperature,
    Speed,

    // Temperature
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Callbacks {
    pub fn as_str(&self) -> &'static str {
        match self {
            Callbacks::Start => "start",
            Callbacks::Today => "today",
            Callbacks::Tomorrow => "tomorrow",
            Callbacks::SettingsHub => "settings-hub",
            Callbacks::SelectLanguage => "select-language",
            Callbacks::SelectUnits => "select-units",
            Callbacks::English => "english",
            Callbacks::Ukrainian => "ukrainian",
            Callbacks::Deutsch => "deutsch",
            Callbacks::Temperature => "temperature",
            Callbacks::Speed => "speed",
            Callbacks::Celsius => "celsius",
            Callbacks::Fahrenheit => "fahrenheit",
            Callbacks::Kelvin => "kelvin",
        }
    }
}
