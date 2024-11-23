use floria_plugin_sdk::data::*;

/// Construct a scalar.
pub fn scalar(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(arguments.clone().into())
}
