use floria_plugin_sdk::data::*;

/// The $xor function takes two Boolean arguments. It evaluates to false if both arguments either
/// evaluate to true or both arguments evaluate to false, and evaluates to true otherwise.
pub fn xor(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 boolean arguments".into());
    }

    let arg1 = match arguments.first().expect("first") {
        Any::Boolean(boolean) => *boolean,
        _ => return Err("first argument is not a boolean".into()),
    };

    let arg2 = match arguments.get(1).expect("second") {
        Any::Boolean(boolean) => *boolean,
        _ => return Err("second argument is not a boolean".into()),
    };

    Ok((arg1 != arg2).into())
}
