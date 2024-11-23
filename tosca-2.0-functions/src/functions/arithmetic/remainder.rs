use floria_plugin_sdk::data::*;

/// The $remainder function takes two arguments where the first argument is of an integer, or
/// scalar type and the second argument is of an integer. The result is of the same type as the
/// first argument and its value is the remainder of the division to the second argument.
pub fn remainder(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
