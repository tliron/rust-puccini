use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// Evaluate an expression.
pub fn evaluate(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 1 {
        return Err("must have one argument".into());
    }

    arguments.remove(0).evaluate(&call_site)
}
