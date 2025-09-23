pub enum Callbacks {
    Start,
    Today,
    Tomorrow,
}

impl Callbacks {
    pub fn as_str(&self) -> &'static str {
        match self {
            Callbacks::Start => "start",
            Callbacks::Today => "today",
            Callbacks::Tomorrow => "tomorrow",
        }
    }
}
