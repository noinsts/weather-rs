use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

use crate::states::State;

/// Dialogue type alias.
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

/// Standard result type alias for bot handlers.
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
