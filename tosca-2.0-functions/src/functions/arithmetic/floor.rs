use floria_plugin_sdk::data::*;

/// The $floor function takes a float argument. The result is an integer with the closest value
/// that is less or equal to the value of the float argument.
pub fn floor(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
