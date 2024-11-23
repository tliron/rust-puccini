use floria_plugin_sdk::data::*;

/// The $quotient function takes two arguments where the first argument is of an integer, float, or
/// scalar type and the second argument is of an integer or float type. The result is of
///
/// * A scalar type if the first argument is a scalar, and its value is the arithmetic division of
///   the first argument value by the second argument value. If necessary, the result might be
///   truncated, as decided by the implementation.
///
/// * A float if the first argument is an integer or a float. Note that to transform the float to
///   an integer a round or ceil or floor function must be used.
pub fn quotient(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
