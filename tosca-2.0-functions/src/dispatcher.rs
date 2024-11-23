use super::functions::*;

use floria_plugin_sdk::{data::*, dispatch_bindings::*, export_dispatcher};

//
// Dispatcher
//

/// Dispatcher.
pub struct Dispatcher;

export_dispatcher!(Dispatcher);

impl Guest for Dispatcher {
    type ListResource = List;
    type MapResource = Map;
    type CustomResource = Custom;
    type CallResource = Call;

    fn dispatch(name: String, arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
        match &*name {
            // Graph
            "get_input" => get_input(arguments, call_site),
            "get_property" => get_property(arguments, call_site),
            "get_attribute" => get_attribute(arguments, call_site),
            "get_artifact" => get_artifact(arguments, call_site),
            "value" => value(arguments, call_site),
            "node_index" => node_index(arguments, call_site),
            "relationship_index" => relationship_index(arguments, call_site),
            "available_allocation" => available_allocation(arguments, call_site),
            "select_capability" => select_capability(arguments, call_site),

            // Boolean logic
            "and" => and(arguments, call_site),
            "or" => or(arguments, call_site),
            "not" => not(arguments, call_site),
            "xor" => xor(arguments, call_site),

            // Boolean comparison
            "equal" => equal(arguments, call_site),
            "greater_than" => greater_than(arguments, call_site),
            "greater_or_equal" => greater_or_equal(arguments, call_site),
            "less_than" => less_than(arguments, call_site),
            "less_or_equal" => less_or_equal(arguments, call_site),
            "valid_values" => valid_values(arguments, call_site),
            "matches" => matches(arguments, call_site),

            // Boolean collection
            "has_suffix" => has_suffix(arguments, call_site),
            "has_prefix" => has_prefix(arguments, call_site),
            "contains" => contains(arguments, call_site),
            "has_entry" => has_entry(arguments, call_site),
            "has_key" => has_key(arguments, call_site),
            "has_all_entries" => has_all_entries(arguments, call_site),
            "has_all_keys" => has_all_keys(arguments, call_site),
            "has_any_entry" => has_any_entry(arguments, call_site),
            "has_any_key" => has_any_key(arguments, call_site),

            // Collection
            "length" => length(arguments, call_site),
            "concat" => concat(arguments, call_site),
            "join" => join(arguments, call_site),
            "token" => token(arguments, call_site),

            // Set
            "union" => union(arguments, call_site),
            "intersection" => intersection(arguments, call_site),

            // Arithmetic
            "sum" => sum(arguments, call_site),
            "difference" => difference(arguments, call_site),
            "product" => product(arguments, call_site),
            "quotient" => quotient(arguments, call_site),
            "remainder" => remainder(arguments, call_site),
            "round" => round(arguments, call_site),
            "floor" => floor(arguments, call_site),
            "ceil" => ceil(arguments, call_site),

            // Puccini
            "_evaluate" => evaluate(arguments, call_site),
            "_assert" => assert(arguments, call_site),
            "_apply" => apply(arguments, call_site),
            "_schema" => schema(arguments, call_site),

            _ => Err("unsupported function".into()),
        }
    }
}
