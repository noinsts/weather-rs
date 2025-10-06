use unic_langid::LanguageIdentifier;

use crate::enums::Callbacks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Languages {
    En,
    Uk,
    De,
}

impl Languages {
    pub fn as_str(&self) -> &'static str {
        match self {
            Languages::En => "en",
            Languages::Uk => "uk",
            Languages::De => "de",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Languages::En),
            "uk" => Some(Languages::Uk),
            "de" => Some(Languages::De),
            &_ => todo!(),
        }
    }

    pub fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Languages::Uk => "uk".parse().unwrap(),
            Languages::En => "en".parse().unwrap(),
            Languages::De => "de".parse().unwrap(),
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Languages::Uk => "locales/uk.ftl",
            Languages::En => "locales/en.ftl",
            Languages::De => "locales/de.ftl",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Languages::Uk => "🇺🇦 | Українська",
            Languages::En => "🇺🇸 | English",
            Languages::De => "🇩🇪 | Deutsch"
        }
    }

    pub fn callback(&self) -> Callbacks {
        match self {
            Languages::Uk => Callbacks::Ukrainian,
            Languages::En => Callbacks::English,
            Languages::De => Callbacks::Deutsch,

        }
    }
    
    pub fn all() -> &'static [Languages] {
        &[Languages::Uk, Languages::En, Languages::De]
    }
}

impl Default for Languages {
    fn default() -> Self {
        Languages::Uk
    }
}
