use floria_plugin_sdk::data::*;

/// Construct a list.
pub fn list(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(arguments.clone().into())
}
