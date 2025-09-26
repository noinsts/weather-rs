use std::any::Any;

use teloxide::types::{Message, CallbackQuery};

/// A trait that provides a common interface for extracting identifiers
///
/// Implemented for:
/// - ['Message'] (regular message)
/// - ['CallbackQuery'] (button presses)
pub trait ChatSource {
    /// Returns the chat ID
    fn chat_id(&self) -> i64;

    /// Returns the message ID if available.
    fn message_id(&self) -> Option<i32>;

    /// Returns the user ID if available.
    fn user_id(&self) -> i64;

    /// Allows treating the object as a dynamic type.
    fn is_any(&self) -> &dyn Any;
}

impl ChatSource for Message {
    fn chat_id(&self) -> i64 {
        self.chat.id.0
    }

    fn message_id(&self) -> Option<i32> {
        Some(self.id.0)
    }

    fn user_id(&self) -> i64 {
        self.from()
            .map(|user| user.id.0 as i64)
            .unwrap_or_default()
    }

    fn is_any(&self) -> &dyn Any {
        self
    }
}

impl ChatSource for CallbackQuery {
    fn chat_id(&self) -> i64 {
        self.message
            .as_ref()
            .map(|msg| msg.chat().id.0)
            .unwrap_or_default()
    }

    fn message_id(&self) -> Option<i32> {
        self.message
            .as_ref()
            .map(|msg| msg.id().0)
    }

    fn user_id(&self) -> i64 {
        self.from.id.0 as i64
    }

    fn is_any(&self) -> &dyn Any {
        self
    }
}
