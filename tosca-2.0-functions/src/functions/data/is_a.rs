use floria_plugin_sdk::data::*;

/// Is of a type.
pub fn is_a(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have two arguments".into());
    }

    let value = arguments.first().expect("first argument");

    let Any::Text(type_name) = arguments.get(1).expect("second argument") else {
        return Err("second argument is not a string".into());
    };

    match type_name.as_str() {
        "string" => Ok(matches!(value, Any::Text(_)).into()),
        "integer" => Ok(matches!(value, Any::Integer(_) | Any::UnsignedInteger(_)).into()),
        "float" => Ok(matches!(value, Any::Float(_)).into()),
        "boolean" => Ok(matches!(value, Any::Boolean(_)).into()),
        "bytes" => Ok(matches!(value, Any::Blob(_)).into()),
        "nil" => Ok(matches!(value, Any::Null).into()),
        "timestamp" => Ok(matches!(value, Any::NestedMap(_)).into()), // todo
        "scalar" => Ok(matches!(value, Any::NestedMap(_)).into()),    // todo
        "version" => Ok(matches!(value, Any::NestedMap(_)).into()),   // todo
        "list" => Ok(matches!(value, Any::NestedList(_)).into()),
        "map" => Ok(matches!(value, Any::NestedMap(_)).into()),

        _ => Err(format!("unsupported type: {}", type_name)),
    }
}
