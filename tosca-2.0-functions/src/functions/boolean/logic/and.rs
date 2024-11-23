use floria_plugin_sdk::data::*;

/// The $and function takes two or more Boolean arguments. It evaluates to true if all its
/// arguments evaluate to true. It evaluates to false in all other cases.
pub fn and(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() < 2 {
        return Err("must have at least 2 boolean arguments".into());
    }

    for argument in arguments {
        match argument {
            Any::Boolean(boolean) => {
                if !*boolean {
                    return Ok(false.into());
                }
            }

            _ => return Err("argument is not a boolean".into()),
        }
    }

    Ok(true.into())
}
