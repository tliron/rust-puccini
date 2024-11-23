use super::functions::*;

use floria_plugin_sdk::{data::*, dispatch_bindings::*, export_dispatcher};

//
// Dispatcher
//

/// Dispatcher.
pub struct Dispatcher;

export_dispatcher!(Dispatcher);

impl Guest for Dispatcher {
    type NestedList = List;
    type NestedMap = Map;

    fn dispatch(name: String, arguments: Vec<Any>, site: Site) -> Result<Any, Error> {
        match &*name {
            // Graph
            "tosca:2.0:get_input" => get_input(&arguments, &site),
            "tosca:2.0:get_property" => get_property(&arguments, &site),
            "tosca:2.0:get_attribute" => get_attribute(&arguments, &site),
            "tosca:2.0:get_artifact" => get_artifact(&arguments, &site),
            "tosca:2.0:value" => value(&arguments, &site),
            "tosca:2.0:node_index" => node_index(&arguments, &site),
            "tosca:2.0:relationship_index" => relationship_index(&arguments, &site),
            "tosca:2.0:available_allocation" => available_allocation(&arguments, &site),
            "tosca:2.0:select_capability" => select_capability(&arguments, &site),

            // Boolean logic
            "tosca:2.0:and" => and(&arguments, &site),
            "tosca:2.0:or" => or(&arguments, &site),
            "tosca:2.0:not" => not(&arguments, &site),
            "tosca:2.0:xor" => xor(&arguments, &site),

            // Boolean comparison
            "tosca:2.0:equal" => equal(&arguments, &site),
            "tosca:2.0:greater_than" => greater_than(&arguments, &site),
            "tosca:2.0:greater_or_equal" => greater_or_equal(&arguments, &site),
            "tosca:2.0:less_than" => less_than(&arguments, &site),
            "tosca:2.0:less_or_equal" => less_or_equal(&arguments, &site),
            "tosca:2.0:valid_values" => valid_values(&arguments, &site),
            "tosca:2.0:matches" => matches(&arguments, &site),

            // Boolean collection
            "tosca:2.0:has_suffix" => has_suffix(&arguments, &site),
            "tosca:2.0:has_prefix" => has_prefix(&arguments, &site),
            "tosca:2.0:contains" => contains(&arguments, &site),
            "tosca:2.0:has_entry" => has_entry(&arguments, &site),
            "tosca:2.0:has_key" => has_key(&arguments, &site),
            "tosca:2.0:has_all_entries" => has_all_entries(&arguments, &site),
            "tosca:2.0:has_all_keys" => has_all_keys(&arguments, &site),
            "tosca:2.0:has_any_entry" => has_any_entry(&arguments, &site),
            "tosca:2.0:has_any_key" => has_any_key(&arguments, &site),

            // Boolean data.
            "tosca:2.0:_is_a" => is_a(&arguments, &site),

            // Collection
            "tosca:2.0:length" => length(&arguments, &site),
            "tosca:2.0:concat" => concat(&arguments, &site),
            "tosca:2.0:join" => join(&arguments, &site),
            "tosca:2.0:token" => token(&arguments, &site),

            // Set
            "tosca:2.0:union" => union(&arguments, &site),
            "tosca:2.0:intersection" => intersection(&arguments, &site),

            // Arithmetic
            "tosca:2.0:sum" => sum(&arguments, &site),
            "tosca:2.0:difference" => difference(&arguments, &site),
            "tosca:2.0:product" => product(&arguments, &site),
            "tosca:2.0:quotient" => quotient(&arguments, &site),
            "tosca:2.0:remainder" => remainder(&arguments, &site),
            "tosca:2.0:round" => round(&arguments, &site),
            "tosca:2.0:floor" => floor(&arguments, &site),
            "tosca:2.0:ceil" => ceil(&arguments, &site),

            _ => {
                return Err(Error::new(name, &arguments, site, "unsupported function".into()));
            }
        }
        .map_err(|error| Error::new(name, &arguments, site, error))
    }
}
