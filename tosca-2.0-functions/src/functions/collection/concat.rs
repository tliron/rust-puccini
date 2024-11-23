use floria_plugin_sdk::data::*;

/// The $concat function takes one or more arguments of either the type string or the type list
/// with the same type of their entry_schema. In the case of strings, it returns a string which is
/// the concatenation of the argument strings. In the case of lists, it returns a list that
/// contains all the entries of all the argument lists. Order is preserved both for strings and
/// lists. This function does not recurse into the entries of the lists.
pub fn concat(_arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    Ok(true.into())
}
