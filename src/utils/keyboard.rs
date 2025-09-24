use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::enums::Callbacks;

pub fn get_hub_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Today", Callbacks::Today.as_str())],
        vec![InlineKeyboardButton::callback("Tomorrow", Callbacks::Tomorrow.as_str())],
    ])
}

pub fn get_to_hub() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Back", Callbacks::Start.as_str())],
    ])
}