use super::super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// The $and function takes two or more Boolean arguments. It evaluates to true if all its
/// arguments evaluate to true. It evaluates to false in all other cases.
pub fn and(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() < 2 {
        return Err("must have at least 2 boolean arguments".into());
    }

    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?;

        let Expression::Boolean(argument) = argument else {
            return Err(format!("argument not a boolean: {}", argument.type_name()));
        };

        if !argument {
            return Ok(Some(false.into()));
        }
    }

    Ok(Some(true.into()))
}
