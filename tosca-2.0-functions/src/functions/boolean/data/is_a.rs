use floria_plugin_sdk::data::*;

/// Is of a type.
pub fn is_a(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 1 {
        return Err("must have one string argument".into());
    }

    let Any::Text(type_name) = arguments.first().expect("first") else {
        return Err("argument is not a string".into());
    };

    match type_name.as_str() {
        "string" => {}
        "integer" => {}
        "float" => {}
        "boolean" => {}
        "bytes" => {}
        "nil" => {}
        "timestamp" => {}
        "scalar" => {}
        "version" => {}
        "list" => {}
        "map" => {}

        _ => return Err(format!("unsupported type: {}", type_name)),
    }

    Ok(true.into())
}
