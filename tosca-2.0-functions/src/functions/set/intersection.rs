use floria_plugin_sdk::data::*;

/// The $intersection function takes one or more list arguments, all having the entry schema of the
/// same type. The result is a list that contains all entries that can be found in each of the
/// argument lists.
pub fn intersection(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(None)
}
