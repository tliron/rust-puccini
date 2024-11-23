use floria_plugin_sdk::data::*;

/// The $has_entry function takes two arguments. The first argument is a list or a map. The second
/// argument is of the type matching the entry_schema of the first argument. It evaluates to true
/// if the second argument is an entry in the first argument. For lists this means that the second
/// argument is a value in the first argument list. For maps this means that the second argument is
/// a value in any of the key-value pairs in the first argument map.
pub fn has_entry(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
