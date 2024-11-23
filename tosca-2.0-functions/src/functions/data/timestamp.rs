use floria_plugin_sdk::data::*;

/// Construct a timestamp.
pub fn timestamp(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(arguments.clone().into())
}
