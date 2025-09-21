use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn get_hub_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Button 1", "today")],
        vec![InlineKeyboardButton::callback("Button 2", "tomorrow")],
    ])
}