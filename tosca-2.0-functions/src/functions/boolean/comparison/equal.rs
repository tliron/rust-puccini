use super::super::super::{super::data::*, puccini::*};

use floria_plugin_sdk::data::*;

/// The $equal function takes two arguments that have the same type. It evaluates to true if the
/// arguments are equal. An $equal function that uses arguments of different types SHOULD be
/// flagged as an error.
pub fn equal(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let left = arguments.remove(0).must_evaluate(&call_site)?;
    let right = arguments.remove(0).must_evaluate(&call_site)?;

    let left = left.coerce_if_custom(&right)?;
    let right = right.coerce_if_custom(&left)?;

    if !left.same_type(&right) {
        return Err(format!("arguments must be of the same type: {}, {}", left.type_name(), right.type_name()));
    }

    // Note: equal can work on any type!

    Ok(Some(
        if left == right {
            true
        } else {
            set_assert_reason(Some(format!("{} = {}", left, right)))?;
            false
        }
        .into(),
    ))
}
