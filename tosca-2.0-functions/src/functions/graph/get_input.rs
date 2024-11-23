use floria_plugin_sdk::data::*;

/// The $get_input function is used to retrieve the values of parameters declared within the inputs
/// section of a TOSCA service template.
pub fn get_input(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(Any::Null)
}
