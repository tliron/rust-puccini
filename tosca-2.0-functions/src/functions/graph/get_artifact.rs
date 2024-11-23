use floria_plugin_sdk::data::*;

/// The $get_artifact function is used to retrieve the location of artifacts defined by modelable
/// entities in a service template.
pub fn get_artifact(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(Any::Null)
}
