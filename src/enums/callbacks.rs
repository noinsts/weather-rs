pub enum Callbacks {
    Start,
    Today,
    Tomorrow,

    // Settings
    SettingsHub,
    SelectLanguage,
    
    // Languages
    English,
    Ukrainian,
    Deutsch,
}

impl Callbacks {
    pub fn as_str(&self) -> &'static str {
        match self {
            Callbacks::Start => "start",
            Callbacks::Today => "today",
            Callbacks::Tomorrow => "tomorrow",
            Callbacks::SettingsHub => "settings-hub",
            Callbacks::SelectLanguage => "select-language",
            Callbacks::English => "english",
            Callbacks::Ukrainian => "ukrainian",
            Callbacks::Deutsch => "deutsch"
        }
    }
}
