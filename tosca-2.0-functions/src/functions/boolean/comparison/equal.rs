use floria_plugin_sdk::data::*;

/// The $equal function takes two arguments that have the same type. It evaluates to true if the
/// arguments are equal. An $equal function that uses arguments of different types SHOULD be
/// flagged as an error.
pub fn equal(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let arg1 = arguments.first().expect("first");
    let arg2 = arguments.get(1).expect("second");

    if !arg1.same_type(arg2) {
        return Err("arguments must be of the same type".into());
    }

    Ok((arg1 == arg2).into())
}
