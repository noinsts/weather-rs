use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use crate::states::State;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
