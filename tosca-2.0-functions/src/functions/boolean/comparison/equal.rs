use floria_plugin_sdk::data::*;

/// The $equal function takes two arguments that have the same type. It evaluates to true if the
/// arguments are equal. An $equal function that uses arguments of different types SHOULD be
/// flagged as an error.
pub fn equal(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let left = arguments.first().expect("first argument");
    let right = arguments.get(1).expect("second argument");

    if !left.same_type(right) {
        return Err(format!("arguments must be of the same type: {}, {}", left.type_name(), right.type_name()));
    }

    Ok((left == right).into())
}
