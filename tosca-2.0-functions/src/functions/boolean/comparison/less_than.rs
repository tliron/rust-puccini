use floria_plugin_sdk::data::*;

/// The $less_than function takes two arguments of integer, float, string, timestamp, version, any
/// scalar type, or their derivations. It evaluates to true if both arguments are of the same type,
/// and if the first argument is less than the second argument and evaluates to false otherwise.
pub fn less_than(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let arg1 = arguments.first().expect("first");
    let arg2 = arguments.get(1).expect("second");

    if !arg1.same_type(arg2) {
        return Err("arguments must be of the same type".into());
    }

    Ok((arg1 < arg2).into())
}
