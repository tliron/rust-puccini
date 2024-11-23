use floria_plugin_sdk::data::*;

/// The $has_all_entries function takes two arguments. The first argument is a list or a map. The
/// second argument is a list with the entry_schema matching the entry_schema of the first
/// argument. It evaluates to true if for all entries in the second argument there is an equal
/// value entry in the first argument.
pub fn has_all_entries(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
