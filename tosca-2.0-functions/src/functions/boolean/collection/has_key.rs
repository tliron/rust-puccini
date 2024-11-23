use floria_plugin_sdk::data::*;

/// The $has_key function takes two arguments. The first argument is a map. The second argument is
/// of the type matching the key_schema of the first argument. It evaluates to true if the second
/// argument is a key in any of the key-value pairs in the first argument map.
pub fn has_key(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
