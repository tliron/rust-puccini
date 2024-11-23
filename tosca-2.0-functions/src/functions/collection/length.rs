use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// The $length function takes an argument of type string, list, or map. It returns the number of
/// nicode characters in the string, or the numbers of values in the list, or the number of
/// key-values pairs in the map.
pub fn length(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 1 {
        return Err("must have one argument".into());
    }

    let value = arguments.remove(0).must_evaluate(&call_site)?;

    let length = match value {
        Expression::Text(text) => text.chars().count(),
        Expression::List(list_resource) => list_resource.list().inner.len(),
        Expression::Map(map_resource) => map_resource.map().inner.len(),

        _ => return Err(format!("argument not a string, list, or map: {}", value.type_name())),
    };

    let length: Result<i64, _> = length.try_into();
    let Ok(length) = length else {
        return Err("length won't fit in integer".into());
    };

    Ok(Some(length.into()))
}
