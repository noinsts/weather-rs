use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;

use crate::enums::{Callbacks, Commands};
use crate::handlers::{receive_city, start, weather, settings};
use crate::states::State;

/// Bot's update handling schema.
///
/// Branches:
/// - Commands
/// - Dialogue
/// - Callback queries
///
/// Returns an ['UpdateHandler'] tree ready for the dispatcher.
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
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::SettingsHub.as_str()))
                .endpoint(settings::hub::handler),
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::SelectLanguage.as_str()))
                .endpoint(settings::language::hub::handler),
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::Ukrainian.as_str()))
                .endpoint(settings::language::ukrainian_handler)
        )
        .branch(
            dptree::filter(|q: CallbackQuery| q.data.as_deref() == Some(Callbacks::English.as_str()))
                .endpoint(settings::language::english_handler)
        );

    dptree::entry()
        .branch(dialogue)
        .branch(callback_queries)
}
