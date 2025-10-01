use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::enums::Callbacks;

/// Returns the main hub keyboard with options for today's and tomorrow's weather.
pub fn get_hub_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Today", Callbacks::Today.as_str())],
        vec![InlineKeyboardButton::callback("Tomorrow", Callbacks::Tomorrow.as_str())],
        vec![InlineKeyboardButton::callback("Settings", Callbacks::SettingsHub.as_str())],
    ])
}

/// Returns a keyboard with a single "back" button that leads to the hub.
pub fn get_to_hub() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Back", Callbacks::Start.as_str())],
    ])
}

/// Returns settings hub keyboard
pub fn get_settings_hub() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Select language", Callbacks::SelectLanguage.as_str())],
        vec![InlineKeyboardButton::callback("Back", Callbacks::Start.as_str())],
    ])
}