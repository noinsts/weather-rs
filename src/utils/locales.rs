use std::fs;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

use crate::enums::languages::Languages;

pub fn get_text(lang: Languages, key: &str, args: Option<&FluentArgs>) -> String {
    let content = fs::read_to_string(lang.path())
        .unwrap_or_else(|_| panic!("{} not found", lang.path()));

    let resource = FluentResource::try_new(content)
        .expect("cannot parse FTL file");

    let lang_id = lang.lang_id();
    let mut bundle = FluentBundle::new(vec![lang_id]);
    bundle.add_resource(resource).unwrap();

    let msg = bundle.get_message(key)
        .unwrap_or_else(|| panic!("{} not found", lang.path()));
    let pattern = msg.value().expect("Message has no value");

    let mut errors = vec![];
    bundle.format_pattern(pattern, args, &mut errors).to_string()
}