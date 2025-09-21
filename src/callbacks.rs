pub enum Callbacks {
    Today,
    Tomorrow,
}

impl Callbacks {
    pub fn as_str(&self) -> &'static str {
        match self {
            Callbacks::Today => "today",
            Callbacks::Tomorrow => "tomorrow",
        }
    }
}
