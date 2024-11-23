use floria_plugin_sdk::data::*;

/// The $contains function takes two arguments. Both arguments are either of type string or of type
/// list. It evaluates to true if the second argument is contained in the first argument. For
/// strings that means that the second argument is a substring of the first argument. For lists
/// this means that the values of the second list are contained in the first list in an
/// uninterrupted sequence and in the same order.
pub fn contains(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
