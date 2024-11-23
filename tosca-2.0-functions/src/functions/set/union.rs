use floria_plugin_sdk::data::*;

/// The $union function takes one or more list arguments, all having the entry schema of the same
/// type. The result is a list that contains all non-duplicate entries from all the argument lists.
/// By non-duplicate is meant that no two entries in the result list are equal.
pub fn union(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(None)
}
