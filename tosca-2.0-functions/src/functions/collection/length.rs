use floria_plugin_sdk::data::*;

/// The $length function takes an argument of type string, list, or map. It returns the number of
/// nicode characters in the string, or the numbers of values in the list, or the number of
/// key-values pairs in the map.
pub fn length(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 1 {
        return Err("must have one argument".into());
    }

    let value = arguments.first().expect("first argument");

    let length = match value {
        Any::Text(text) => text.chars().count(),
        Any::NestedList(list) => list.to_list().inner.len(),
        Any::NestedMap(map) => map.to_map().inner.len(),

        _ => return Err("argument not a string, list, or map".into()),
    };

    let length: Result<i64, _> = length.try_into();
    let Ok(length) = length else {
        return Err("length won't fit in integer".into());
    };

    Ok(length.into())
}
