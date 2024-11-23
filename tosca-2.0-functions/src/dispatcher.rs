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
            "tosca:2.0:get_input" => get_property(&arguments, &site),
            "tosca:2.0:get_property" => get_property(&arguments, &site),
            "tosca:2.0:get_attribute" => get_attribute(&arguments, &site),
            "tosca:2.0:get_artifact" => get_property(&arguments, &site),
            "tosca:2.0:value" => get_property(&arguments, &site),
            "tosca:2.0:node_index" => get_property(&arguments, &site),
            "tosca:2.0:relationship_index" => get_property(&arguments, &site),
            "tosca:2.0:available_allocation" => get_property(&arguments, &site),
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
            "tosca:2.0:valid_values" => get_property(&arguments, &site),
            "tosca:2.0:matches" => get_property(&arguments, &site),

            // Boolean collection
            "tosca:2.0:has_suffix" => get_property(&arguments, &site),
            "tosca:2.0:has_prefix" => get_property(&arguments, &site),
            "tosca:2.0:contains" => get_property(&arguments, &site),
            "tosca:2.0:has_entry" => get_property(&arguments, &site),
            "tosca:2.0:has_key" => get_property(&arguments, &site),
            "tosca:2.0:has_all_entries" => get_property(&arguments, &site),
            "tosca:2.0:has_all_keys" => get_property(&arguments, &site),
            "tosca:2.0:has_any_entry" => get_property(&arguments, &site),
            "tosca:2.0:has_any_key" => get_property(&arguments, &site),

            // Boolean data.
            "tosca:2.0:conforms_to_schema" => conforms_to_schema(&arguments, &site),

            // Collection
            "tosca:2.0:length" => get_property(&arguments, &site),
            "tosca:2.0:concat" => get_property(&arguments, &site),
            "tosca:2.0:join" => get_property(&arguments, &site),
            "tosca:2.0:token" => get_property(&arguments, &site),

            // Set
            "tosca:2.0:union" => get_property(&arguments, &site),
            "tosca:2.0:intersection" => get_property(&arguments, &site),

            // Arithmetic
            "tosca:2.0:sum" => get_property(&arguments, &site),
            "tosca:2.0:difference" => get_property(&arguments, &site),
            "tosca:2.0:product" => get_property(&arguments, &site),
            "tosca:2.0:quotient" => get_property(&arguments, &site),
            "tosca:2.0:remainder" => get_property(&arguments, &site),
            "tosca:2.0:round" => get_property(&arguments, &site),
            "tosca:2.0:floor" => get_property(&arguments, &site),
            "tosca:2.0:ceil" => get_property(&arguments, &site),

            _ => {
                return Err(Error::new(name, &arguments, site, "unknown function".into()));
            }
        }
        .map_err(|error| Error::new(name, &arguments, site, error))
    }
}
