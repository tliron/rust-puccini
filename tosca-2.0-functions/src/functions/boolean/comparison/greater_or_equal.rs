use super::super::super::{super::data::*, puccini::*};

use floria_plugin_sdk::data::*;

/// The $greater_or_equal function takes two arguments of integer, float, string, timestamp,
/// version, any scalar type, or their derivations. It evaluates to true if both arguments are of
/// the same type, and if the first argument is greater than or equal to the second argument and
/// evaluates to false otherwise.
pub fn greater_or_equal(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
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

    let left = left.comparator()?;
    let right = right.comparator()?;

    Ok(Some(
        if left >= right {
            true
        } else {
            set_assert_reason(Some(format!("{} >= {}", left, right)))?;
            false
        }
        .into(),
    ))
}
