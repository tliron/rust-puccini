#![allow(unused)]

use super::parser::*;

use floria_plugin_sdk::data::*;

/// Follow TOSCA path.
pub fn follow_tosca_path(path_site: Entity, arguments: &Vec<Expression>) -> Result<Entity, String> {
    let mut parser = ToscaPathParser::new(arguments);
    parser.next_site(path_site)
}

/// Follow TOSCA path to property value.
pub fn follow_tosca_path_to_property_value(
    path_site: Entity,
    arguments: &Vec<Expression>,
    read_only: bool,
) -> Result<Expression, String> {
    let mut parser = ToscaPathParser::new(arguments);
    let site = parser.next_site(path_site)?;
    let property = parser.next_property(&site, read_only)?;
    parser.next_expression(&property)
}
