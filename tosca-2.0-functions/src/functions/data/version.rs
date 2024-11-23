use floria_plugin_sdk::data::*;

/// Construct a version.
pub fn version(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(arguments.clone().into())
}
