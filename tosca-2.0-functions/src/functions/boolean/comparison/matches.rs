use super::super::super::{super::data::*, puccini::*};

use {floria_plugin_sdk::data::*, regex::*};

/// The $matches function takes two arguments. The first argument is a general string, and the
/// second argument is a string that encodes a regular expression pattern. It evaluates to true if
/// the first argument matches the regular expression pattern represented by the second argument
/// and false otherwise.
pub fn matches(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 2 {
        return Err("must have 2 string arguments".into());
    }

    let Expression::Text(string) = arguments.remove(0).must_evaluate(&call_site)? else {
        return Err("first argument must be a string".into());
    };

    let Expression::Text(pattern) = arguments.remove(0).must_evaluate(&call_site)? else {
        return Err("second argument must be a string".into());
    };

    let regex = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(error) => return Err(error.to_string()),
    };

    Ok(Some(
        if regex.is_match(&string) {
            true
        } else {
            set_assert_reason(Some(format!("{} =~ {}", string, pattern)))?;
            false
        }
        .into(),
    ))
}
