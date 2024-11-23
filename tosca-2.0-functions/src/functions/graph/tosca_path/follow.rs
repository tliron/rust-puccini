#![allow(unused)]

use super::parser::*;

use floria_plugin_sdk::data::*;

/// Follow TOSCA path.
pub fn follow_tosca_path(site: Entity, arguments: &Vec<Any>) -> Result<Entity, String> {
    let mut parser = ToscaPathParser::new(arguments);
    parser.next_site(site)
}

/// Follow TOSCA path to property value.
pub fn follow_tosca_path_to_property_value(site: Entity, arguments: &Vec<Any>, read_only: bool) -> Result<Any, String> {
    let mut parser = ToscaPathParser::new(arguments);
    let site = parser.next_site(site)?;
    let property = parser.next_property(&site, read_only)?;
    parser.next_any(&property)
}
