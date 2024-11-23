use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// Coerce to a schema.
///
/// If the value is valid, meaning that it adheres to the schema, will return a canonicalized
/// version of the value, or [None] if already canonical. Otherwise, will return an error.
pub fn schema(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 2 {
        return Err("must have two arguments".into());
    }

    let value = arguments.remove(0).evaluate(&call_site)?;
    let schema: Schema = arguments.remove(0).must_evaluate(&call_site)?.try_into()?;

    schema.coerce_option(value)
}
