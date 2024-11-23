use floria_plugin_sdk::data::*;

/// This function is used to return the runtime index of the current node representation in the
/// list of node representations created from the same node template. The first index is 0, which
/// is also what $node_index will return when a single node representation is created from a node
/// template (i.e. where the default count is 1). The function should not be used outside a valid
/// node context.
pub fn node_index(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(None)
}
