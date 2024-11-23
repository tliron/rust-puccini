use floria_plugin_sdk::data::*;

/// The $matches function takes two arguments. The first argument is a general string, and the
/// second argument is a string that encodes a regular expression pattern. It evaluates to true if
/// the first argument matches the regular expression pattern represented by the second argument
/// and false otherwise.
pub fn matches(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
