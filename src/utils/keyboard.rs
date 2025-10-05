use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::enums::Callbacks;
use crate::enums::languages::Languages;
use crate::utils::locales::get_text;

/// Returns the main hub keyboard with options for today's and tomorrow's weather.
///
/// # Arguments
/// - `lang` - мова кнопок клавіатури
///
/// # Returns
/// - `InlineKeyboardMarkup` - inline keyboard
pub fn get_hub_keyboard(lang: Languages) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(get_text(lang, "today", None), Callbacks::Today.as_str())],
        vec![InlineKeyboardButton::callback(get_text(lang, "tomorrow", None), Callbacks::Tomorrow.as_str())],
        vec![InlineKeyboardButton::callback(get_text(lang, "settings", None), Callbacks::SettingsHub.as_str())],
    ])
}

/// Returns a keyboard with a single "back" button that leads to the hub.
pub fn get_to_hub(lang: Languages) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(get_text(lang, "back", None), Callbacks::Start.as_str())],
    ])
}

/// Returns settings hub keyboard
pub fn get_settings_hub(lang: Languages) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(get_text(lang, "select-language", None), Callbacks::SelectLanguage.as_str())],
        vec![InlineKeyboardButton::callback(get_text(lang, "back", None), Callbacks::Start.as_str())],
    ])
}

/// Returns keyboard with languages
pub fn get_languages_keyboard(lang: Languages) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("🇺🇸 | English", Callbacks::English.as_str()),
             InlineKeyboardButton::callback("🇺🇦 | Українська", Callbacks::Ukrainian.as_str())],
        vec![InlineKeyboardButton::callback(get_text(lang, "back", None), Callbacks::SettingsHub.as_str())],
    ])
}