use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::callbacks::Callbacks;

pub fn get_hub_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Button 1", Callbacks::Today.as_str())],
        vec![InlineKeyboardButton::callback("Button 2", Callbacks::Tomorrow.as_str())],
    ])
}