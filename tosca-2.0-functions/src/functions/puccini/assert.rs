use super::super::super::data::*;

use {floria_plugin_sdk::data::*, std::sync::*};

/// Error if argument is not true.
pub fn assert(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    if arguments.len() != 1 {
        return Err("must have one argument".into());
    }

    set_assert_reason(None)?;

    if let Expression::Boolean(boolean) = arguments.remove(0).must_evaluate(&call_site)?
        && boolean
    {
        Ok(None)
    } else {
        match get_assert_reason()? {
            Some(reason) => Err(format!("invalid because {} is false", reason)),
            None => Err("invalid because an expression is false".into()),
        }
    }
}

/// Assert reason.
static ASSERT_REASON: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Default::default());

/// Get assert reason.
pub fn get_assert_reason() -> Result<Option<String>, String> {
    Ok(ASSERT_REASON.lock().map_err(|error| error.to_string())?.clone())
}

/// Set assert reason.
pub fn set_assert_reason(value: Option<String>) -> Result<(), String> {
    *ASSERT_REASON.lock().map_err(|error| error.to_string())? = value;
    Ok(())
}
