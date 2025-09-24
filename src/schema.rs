use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;

use crate::enums::{Callbacks, Commands};
use crate::handlers::{receive_city, start, weather};
use crate::states::State;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let commands = teloxide::filter_command::<Commands, _>()
        .branch(case![Commands::Start].endpoint(start::message_handler));

    let dialogue = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .branch(commands)
                .branch(case![State::ReceiveCity].endpoint(receive_city::receive_city_handler))
        );

    let callback_queries = Update::filter_callback_query()
        .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Start.as_str()))
                .endpoint(start::callback_handler),
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Today.as_str()))
                .endpoint(weather::today_handler),
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Tomorrow.as_str()))
                .endpoint(weather::tomorrow_handler),
        );

    dptree::entry()
        .branch(dialogue)
        .branch(callback_queries)
}
