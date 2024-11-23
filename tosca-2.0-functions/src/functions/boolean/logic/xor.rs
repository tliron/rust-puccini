use super::super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// The $xor function takes two Boolean arguments. It evaluates to false if both arguments either
/// evaluate to true or both arguments evaluate to false, and evaluates to true otherwise.
pub fn xor(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 2 {
        return Err("must have 2 boolean arguments".into());
    }

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(left) = argument else {
        return Err(format!("first argument not a boolean: {}", argument.type_name()));
    };

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(right) = argument else {
        return Err(format!("second argument not a boolean: {}", argument.type_name()));
    };

    Ok(Some((left != right).into()))
}
