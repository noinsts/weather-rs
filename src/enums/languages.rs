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
}

impl Default for Languages {
    fn default() -> Self {
        Languages::Uk
    }
}
