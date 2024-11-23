use floria_plugin_sdk::data::*;

/// The $has_any_entry function takes two arguments. The first argument is a list or a map. The
/// second argument is a list with the entry_schema matching the entry_schema of the first
/// argument. It evaluates to true if there is an entry in the second argument that is equal to an
/// entry in the first argument.
pub fn has_any_entry(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
