use floria_plugin_sdk::data::*;

/// The $join function takes either one or two arguments where the first one is of type list of
/// strings and the second (optional) argument is of type string. It returns a string that is the
/// joining of the entries in the first argument while adding an optional delimiter between the
/// strings.
pub fn join(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
