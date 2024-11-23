use floria_plugin_sdk::data::*;

/// The $or function takes two or more Boolean arguments. It evaluates to false if all of its
/// arguments evaluate to false. It evaluates to true in all other cases.
pub fn or(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() < 2 {
        return Err("must have at least 2 boolean arguments".into());
    }

    for argument in arguments {
        match argument {
            Any::Boolean(boolean) => {
                if *boolean {
                    return Ok(true.into());
                }
            }

            _ => return Err("argument is not a boolean".into()),
        }
    }

    Ok(false.into())
}
