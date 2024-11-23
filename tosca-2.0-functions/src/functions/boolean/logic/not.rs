use super::super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// The $not function takes one Boolean argument. It evaluates to true if its argument evaluates to
/// false and evaluates to false if its argument evaluates to true.
pub fn not(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 1 {
        return Err("must have one boolean argument".into());
    }

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(value) = argument else {
        return Err(format!("first argument not a boolean: {}", argument.type_name()));
    };

    Ok(Some((!value).into()))
}
