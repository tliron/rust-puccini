use super::tosca_path::*;

use floria_plugin_sdk::data::*;

/// The $get_attribute function is used within a representation graph to obtain attribute values
/// from nodes and relationships that have been created from an application model described in a
/// service template. The nodes or relationships can be referenced by their name as assigned in the
/// service template or relative to the context where they are being invoked.
pub fn get_attribute(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    let site = call_site.entity()?;
    follow_tosca_path_to_property_value(site, &arguments, false).map(Some)
}
