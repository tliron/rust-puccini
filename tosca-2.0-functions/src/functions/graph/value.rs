use floria_plugin_sdk::data::*;

/// This function is used as an argument inside validation functions. It returns the value of the
/// property, attribute, or parameter for which the validation clause is defined.
pub fn value(_arguments: &Vec<Any>, site: &Site) -> Result<Any, String> {
    if let Some(value) = site.get_property()? {
        return Ok(value);
    }

    Ok(Any::Null)
}
