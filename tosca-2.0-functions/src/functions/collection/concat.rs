use floria_plugin_sdk::data::*;

/// The $concat function takes one or more arguments of either the type string or the type list
/// with the same type of their entry_schema. In the case of strings, it returns a string which is
/// the concatenation of the argument strings. In the case of lists, it returns a list that
/// contains all the entries of all the argument lists. Order is preserved both for strings and
/// lists. This function does not recurse into the entries of the lists.
pub fn concat(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    let mut result: Option<ConcatResult> = None;

    for argument in arguments {
        match argument {
            Any::Text(text) => {
                match &mut result {
                    Some(ConcatResult::String(result)) => result.push_str(text),
                    None => result = Some(ConcatResult::String(text.clone())),
                    Some(ConcatResult::List(_)) => return Err("argument not a list".into()),
                };
            }

            Any::NestedList(list) => {
                match &mut result {
                    Some(ConcatResult::List(result)) => result.extend_from_slice(&list.to_list().inner),
                    None => result = Some(ConcatResult::List(list.to_list().inner.clone())),
                    Some(ConcatResult::String(_)) => return Err("argument not a string".into()),
                };
            }

            _ => return Err("argument not a string or list".into()),
        }
    }

    match result {
        Some(ConcatResult::String(string)) => Ok(string.into()),
        Some(ConcatResult::List(list)) => Ok(list.into()),
        None => Err("no arguments provided".into()),
    }
}

enum ConcatResult {
    String(String),
    List(Vec<Any>),
}
