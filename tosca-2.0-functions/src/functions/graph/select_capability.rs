use super::super::super::data::*;

use {
    floria_plugin_sdk::{data::*, log},
    std::fmt,
};

/// Select capability. Return its Floria vertex ID.
pub fn select_capability(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    log!("select_capability", "{}", expression_vec_to_string(&arguments));

    let constraints = CapabilityConstraints::parse(&arguments)?;

    let site = call_site.entity()?;
    let service = site.find_tosca_service()?;

    let mut index = 0;
    for vertex_id in service.get_contained_vertex_ids()? {
        let node = Entity::get(&vertex_id)?;
        if node.get_tosca_entity_kind()? == "NodeTemplate" {
            let (node_type_matches, node_template_matches, index_matches) = constraints.match_node(&node, index)?;

            if node_type_matches && node_template_matches && index_matches {
                for vertex_id in node.get_contained_vertex_ids()? {
                    let vertex = Entity::get(&vertex_id)?;
                    if vertex.get_tosca_entity_kind()? == "CapabilityAssignment" {
                        if constraints.match_capability(&vertex)? {
                            return Ok(Some(vertex.id.to_string().into()));
                        }
                    }
                }
            }

            if node_template_matches {
                index += 1;
            }
        }
    }

    Err(format!("capability not found: {}", constraints))
}

//
// CapabilityConstraints
//

#[derive(Debug, Default)]
struct CapabilityConstraints<'own> {
    capability_type_id: Option<Id>,
    capability_name: Option<&'own str>,

    node_type_id: Option<Id>,
    node_template: Option<(&'own str, usize)>,
}

impl<'own> CapabilityConstraints<'own> {
    fn parse(arguments: &'own Vec<Expression>) -> Result<Self, String> {
        let mut find_node = Self::default();

        if let Some(argument) = arguments.first() {
            match argument {
                Expression::Map(map_resource) => {
                    let argument = map_resource.map();

                    if let Some(capability_type_name) = argument.into_get("capability_type_name") {
                        match capability_type_name {
                            Expression::Text(capability_type_name) => {
                                find_node.capability_type_id = Some(Id::parse(Kind::Class, capability_type_name))
                            }

                            _ => {
                                return Err(format!(
                                    "capability_type_name not a string: {}",
                                    capability_type_name.type_name()
                                ));
                            }
                        }
                    }

                    if let Some(capability_name) = argument.into_get("capability_name") {
                        match capability_name {
                            Expression::Text(capability_name) => find_node.capability_name = Some(capability_name),

                            _ => {
                                return Err(format!("capability_name not a string: {}", capability_name.type_name()));
                            }
                        }
                    }

                    if let Some(node_type_name) = argument.into_get("node_type_name") {
                        match node_type_name {
                            Expression::Text(node_type_name) => {
                                let mut id = Id::parse(Kind::Class, node_type_name);
                                id.directory.insert(0, "node".into());
                                id.directory.insert(0, "tosca".into());
                                find_node.node_type_id = Some(id)
                            }

                            _ => {
                                return Err(format!("node_type_name not a string: {}", node_type_name.type_name()));
                            }
                        }
                    }

                    if let Some(node_template_name) = argument.into_get("node_template_name") {
                        match node_template_name {
                            Expression::Text(node_template_name) => {
                                let node_template_index = argument
                                    .inner
                                    .get(&"node_template_index".into())
                                    .ok_or_else(|| "missing node_template_index")?;

                                let node_template_index = match node_template_index {
                                    Expression::Integer(integer) => *integer as usize,
                                    Expression::UnsignedInteger(unsigned_integer) => *unsigned_integer as usize,
                                    _ => {
                                        return Err(format!(
                                            "node_template_index not an integer: {}",
                                            node_template_index.type_name()
                                        ));
                                    }
                                };

                                find_node.node_template = Some((node_template_name, node_template_index));
                            }

                            _ => {
                                return Err(format!(
                                    "node_template_name not a string: {}",
                                    node_template_name.type_name()
                                ));
                            }
                        }
                    }
                }

                _ => {
                    return Err(format!("argument not a map: {}", argument.type_name()));
                }
            }
        }

        Ok(find_node)
    }

    fn match_capability(&self, entity: &Entity) -> Result<bool, String> {
        let mut capability_matches = match &self.capability_type_id {
            Some(capability_type_id) => entity.is_in_class(capability_type_id)?,
            None => true,
        };

        if capability_matches {
            capability_matches = match self.capability_name {
                Some(capability_name) => entity.is_tosca_capability(capability_name)?,
                None => true,
            };
        }

        Ok(capability_matches)
    }

    fn match_node(&self, entity: &Entity, index: usize) -> Result<(bool, bool, bool), String> {
        let node_type_matches = match &self.node_type_id {
            Some(node_type_id) => entity.is_in_class(node_type_id)?,
            None => true,
        };

        let (node_template_matches, index_matches) = if node_type_matches {
            match self.node_template {
                Some((node_template_name, index_)) => {
                    if entity.is_tosca_node(node_template_name)? {
                        (true, index == index_)
                    } else {
                        (false, false)
                    }
                }

                None => (true, true),
            }
        } else {
            (false, false)
        };

        Ok((node_type_matches, node_template_matches, index_matches))
    }
}

impl<'own> fmt::Display for CapabilityConstraints<'own> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separate = false;

        if let Some(capability_type_id) = &self.capability_type_id {
            write!(formatter, "capability_type_id: {}", capability_type_id)?;
            separate = true;
        }

        if let Some(capability_name) = &self.capability_name {
            if separate {
                write!(formatter, ", ")?;
            }
            separate = true;
            write!(formatter, "capability_name: {}", capability_name)?;
        }

        if let Some(node_type_id) = &self.node_type_id {
            if separate {
                write!(formatter, ", ")?;
            }
            separate = true;
            write!(formatter, "node_type_id: {}", node_type_id)?;
        }

        if let Some((node_template, index)) = &self.node_template {
            if separate {
                write!(formatter, ", ")?;
            }
            write!(formatter, "node_template: {} {}", node_template, index)?;
        }

        Ok(())
    }
}
