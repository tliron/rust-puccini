//use super::super::super::data::*;

use {floria_plugin_sdk::data::*, std::sync::*};

/// This function is used as an argument inside validation functions. It returns the value of the
/// property, attribute, or parameter for which the validation clause is defined.
pub fn value(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if !arguments.is_empty() {
        return Err("must have no arguments".into());
    }

    if let Some(value) = get_call_site_override()? {
        return Ok(Some(value));
    }

    if let Some(value) = call_site.value()? {
        //let value = resolve(&site, value, false)?;
        return Ok(Some(value));
    }

    Err("value not available here".into())
}

/// Call site value override.
static CALL_SITE_OVERRIDE: LazyLock<Mutex<Option<Expression>>> = LazyLock::new(|| Default::default());

/// Get call site value override.
pub fn get_call_site_override() -> Result<Option<Expression>, String> {
    Ok(CALL_SITE_OVERRIDE.lock().map_err(|error| error.to_string())?.clone())
}

/// Set call site value override.
pub fn set_call_site_override(value: Option<Expression>) -> Result<(), String> {
    *CALL_SITE_OVERRIDE.lock().map_err(|error| error.to_string())? = value;
    Ok(())
}
