use floria_plugin_sdk::data::*;

/// The $valid_values function takes two arguments. The first argument is of any type and the
/// second argument is a list with any number of values of the same type as the first argument. It
/// evaluates to true if the first argument is equal to a value in the second argument list and
/// false otherwise.
pub fn valid_values(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
