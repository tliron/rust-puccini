use super::super::{super::data::*, graph::*};

use floria_plugin_sdk::data::*;

/// Returns the call site value (like [`$value`](super::super::graph::value)) while also optionally
/// applying a series of expressions to it.
///
/// Each expression's value is assigned to the call site value *in sequence*, allowing for complex
/// transformations and validations.
///
/// Value-less expressions will not affect the call site value, however they can still return an
/// error, for example if the value is invalid.
pub fn apply(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    set_call_site_override(None)?;

    for preparer in arguments {
        if let Some(expression) = preparer.evaluate(&call_site)? {
            set_call_site_override(Some(expression.clone()))?;
        }
    }

    if let Some(value) = get_call_site_override()? {
        return Ok(Some(value));
    }

    if let Some(value) = call_site.value()? {
        return Ok(Some(value));
    }

    Err("value not available here".into())
}
