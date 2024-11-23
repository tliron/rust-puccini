use floria_plugin_sdk::data::*;

/// The $sum function takes one or more arguments of either integer, float, or scalar type. The
/// result is of the same type as the arguments and its value is the arithmetic sum of the
/// arguments' values.
pub fn sum(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
