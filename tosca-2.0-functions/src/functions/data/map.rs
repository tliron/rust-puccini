use floria_plugin_sdk::data::*;

use std::collections::*;

/// Construct a map.
pub fn map(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() % 2 != 0 {
        return Err("must have even number of arguments".into());
    }

    let mut map = BTreeMap::default();

    let mut iter = arguments.iter();
    while let Some(key) = iter.next()
        && let Some(value) = iter.next()
    {
        map.insert(key.clone(), value.clone());
    }

    Ok(map.into())
}
