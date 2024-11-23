use floria_plugin_sdk::data::*;

/// The $ceil function takes a float argument. The result is an integer with the closest value that
/// is greater or equal to the value of the float argument.
pub fn ceil(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
