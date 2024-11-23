use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// The $concat function takes one or more arguments of either the type string or the type list
/// with the same type of their entry_schema. In the case of strings, it returns a string which is
/// the concatenation of the argument strings. In the case of lists, it returns a list that
/// contains all the entries of all the argument lists. Order is preserved both for strings and
/// lists. This function does not recurse into the entries of the lists.
pub fn concat(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    let mut result: Option<ConcatResult> = None;

    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?;
        let type_name = argument.type_name();

        match argument {
            Expression::Text(text) => {
                match &mut result {
                    Some(ConcatResult::String(result)) => result.push_str(&text),
                    None => result = Some(ConcatResult::String(text)),
                    Some(ConcatResult::List(_)) => return Err(format!("argument not a list: {}", type_name)),
                };
            }

            Expression::List(list_resource) => {
                match &mut result {
                    Some(ConcatResult::List(result)) => result.extend_from_slice(&list_resource.list().inner),
                    None => result = Some(ConcatResult::List(list_resource.list().clone().inner)),
                    Some(ConcatResult::String(_)) => return Err(format!("argument not a string: {}", type_name)),
                };
            }

            _ => return Err(format!("argument not a string or list: {}", type_name)),
        }
    }

    match result {
        Some(ConcatResult::String(string)) => Ok(Some(string.into())),
        Some(ConcatResult::List(list)) => Ok(Some(list.into())),
        None => Err("no arguments provided".into()),
    }
}

enum ConcatResult {
    String(String),
    List(Vec<Expression>),
}
