use floria_plugin_sdk::data::*;

/// The $less_or_equal function takes two arguments of integer, float, string, timestamp, version,
/// any scalar type, or their derivations. It evaluates to true if both arguments are of the same
/// type, and if the first argument is less than or equal to the second argument and evaluates to
/// false otherwise
pub fn less_or_equal(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let left = arguments.first().expect("first argument");
    let right = arguments.get(1).expect("second argument");

    // if !argument1.same_type(argument2) {
    //     return Err(format!(
    //         "arguments must be of the same type: {}, {}",
    //         argument1.type_name(),
    //         argument2.type_name()
    //     ));
    // }

    Ok((left <= right).into())
}
