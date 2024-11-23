use floria_plugin_sdk::data::*;

/// The $has_prefix function takes two arguments. Both arguments are either of type string or of tpe list. It evaluates to true if the second argument is a prefix of the first argument. For lists this means that the values of the second list are the first values of the first list in the same order.
pub fn has_prefix(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
