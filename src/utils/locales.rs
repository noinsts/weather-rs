use std::fs;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

use crate::enums::languages::Languages;

/// Retrieves a localizes message string for a given language and key.
///
/// # Arguments
///
/// - `lang` - The target `Languages` enum value specifying the language.
/// - `key` - The key of the message to retrieve from the FTL file.
/// - `args` - Optional `FluentArgs` for dynamic placeholders in the message.
///
/// # Returns
/// A formatted `String` containing the localizes message.
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

/// Creates a `FluentArgs` instance with multiple key-value pairs in a concise way.
///
/// # Example
/// ```rust
/// let args = fluent_args![
///     "city" => "Kyiv",
///     "temp" => 25,
///      "humidity" => 60,
/// ];
/// ```
#[macro_export]
macro_rules! fluent_args {
    ( $( $k:expr => $v:expr ), *$(,)?) => {{
        let mut args = FluentArgs::new();
        $( args.set($k, $v); )*
        args
    }};
}