use floria_plugin_sdk::data::*;

/// The $difference function takes two arguments of either integer, float, or scalar type. The
/// result is of the same type as the arguments and its value is the arithmetic subtraction of the
/// second argument value from the first argument value.
pub fn difference(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
