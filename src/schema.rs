use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::Message;

use crate::commands::Commands;
use crate::handlers::{receive_city, start};
use crate::states::State;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let commands = teloxide::filter_command::<Commands, _>()
        .branch(case![Commands::Start].endpoint(start::start_handler));

    let dialogue = dialogue::enter::<Message, InMemStorage<State>, State, _>()
        .branch(commands)
        .branch(case![State::ReceiveCity].endpoint(receive_city::receive_city_handler));

    Update::filter_message()
        .branch(dialogue)
}
