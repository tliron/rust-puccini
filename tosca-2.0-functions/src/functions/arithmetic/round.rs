use floria_plugin_sdk::data::*;

/// The $round function takes a float argument. The result is an integer with the closest value to
/// the float argument. Equal value distance is rounded down (e.g. 3.5 is rounded down to 3, while
/// 3.53 is rounded up to 4).
pub fn round(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(true.into()))
}
