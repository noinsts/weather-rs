#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Languages {
    En,
    Uk,
}

impl Languages {
    pub fn as_str(&self) -> &'static str {
        match self { 
            Languages::En => "en",
            Languages::Uk => "uk",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Languages::En),
            "uk" => Some(Languages::Uk),
            &_ => todo!(),
        }
    }
}

impl Default for Languages {
    fn default() -> Self {
        Languages::Uk
    }
}
