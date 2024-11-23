use floria_plugin_sdk::data::*;

/// The $not function takes one Boolean argument. It evaluates to true if its argument evaluates to
/// false and evaluates to false if its argument evaluates to true.
pub fn not(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 1 {
        return Err("must have one boolean argument".into());
    }

    let Any::Boolean(argument) = arguments.first().expect("first argument") else {
        return Err("argument is not a boolean".into());
    };

    Ok((!*argument).into())
}
