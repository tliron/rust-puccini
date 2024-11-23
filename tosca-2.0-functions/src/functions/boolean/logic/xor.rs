use floria_plugin_sdk::data::*;

/// The $xor function takes two Boolean arguments. It evaluates to false if both arguments either
/// evaluate to true or both arguments evaluate to false, and evaluates to true otherwise.
pub fn xor(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 boolean arguments".into());
    }

    let Any::Boolean(argument1) = arguments.first().expect("first") else {
        return Err("first argument is not a boolean".into());
    };

    let Any::Boolean(argument2) = arguments.get(1).expect("second") else {
        return Err("second argument is not a boolean".into());
    };

    Ok((argument1 != argument2).into())
}
