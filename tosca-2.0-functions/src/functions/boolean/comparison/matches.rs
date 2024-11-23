use {floria_plugin_sdk::data::*, regex::*};

/// The $matches function takes two arguments. The first argument is a general string, and the
/// second argument is a string that encodes a regular expression pattern. It evaluates to true if
/// the first argument matches the regular expression pattern represented by the second argument
/// and false otherwise.
pub fn matches(arguments: &Vec<Any>, _site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 string arguments".into());
    }

    let string = match arguments.first().expect("first argument") {
        Any::Text(string) => string,
        _ => return Err("first argument must be a string".into()),
    };

    let pattern = match arguments.get(1).expect("second argument") {
        Any::Text(string) => string,
        _ => return Err("second argument must be a string".into()),
    };

    let regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => return Err(error.to_string()),
    };

    Ok(regex.is_match(string).into())
}
