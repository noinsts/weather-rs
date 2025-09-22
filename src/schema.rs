use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;

use crate::enums::{Callbacks, Commands};
use crate::handlers::{receive_city, start, today};
use crate::states::State;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let commands = teloxide::filter_command::<Commands, _>()
        .branch(case![Commands::Start].endpoint(start::start_handler));

    let dialogue = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .branch(commands)
                .branch(case![State::ReceiveCity].endpoint(receive_city::receive_city_handler))
        );

    let callback_queries = Update::filter_callback_query()
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Today.as_str()))
                .endpoint(today::today_handler),
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Tomorrow.as_str()))
                .endpoint(today::tomorrow_handler),
        );

    dptree::entry()
        .branch(dialogue)
        .branch(callback_queries)
}
