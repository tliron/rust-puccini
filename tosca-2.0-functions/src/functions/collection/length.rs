use floria_plugin_sdk::data::*;

/// The $length function takes an argument of type string, list, or map. It returns the number of
/// nicode characters in the string, or the numbers of values in the list, or the number of
/// key-values pairs in the map.
pub fn length(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
