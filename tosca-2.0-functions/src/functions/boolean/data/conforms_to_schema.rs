use floria_plugin_sdk::data::*;

/// Conforms to schema.
pub fn conforms_to_schema(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
