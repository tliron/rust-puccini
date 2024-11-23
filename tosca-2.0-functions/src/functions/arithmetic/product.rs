use floria_plugin_sdk::data::*;

/// The $product function takes either:
///
/// * Two arguments where the first argument is of a scalar type and the second argument is of an
///   integer or float type. The result is of the same type as the first argument and its value is
///   the arithmetic product of the first argument value and the second argument value.
///
/// * Expression number of arguments of type integer or float. If all inputs are of type integer, then the
///   result is of type integer, otherwise it is of type float. The result value is the arithmetic
///   product of all the arguments values.
pub fn product(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
