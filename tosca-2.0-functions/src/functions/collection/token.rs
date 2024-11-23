use floria_plugin_sdk::data::*;

/// The $token function is used within a TOSCA service template on a string to parse out (tokenize)
/// substrings separated by one or more token characters within a larger string.
pub fn token(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
