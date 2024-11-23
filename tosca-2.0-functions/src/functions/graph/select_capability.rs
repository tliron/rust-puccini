use super::super::super::data::*;

use {
    floria_plugin_sdk::{data::*, log},
    std::fmt,
};

/// Select capability. Return it's Floria node ID.
pub fn select_capability(arguments: &Vec<Any>, site: &Site) -> Result<Any, String> {
    log!("select_capability", "{}", normal_vec_to_string(arguments));

    let constraints = CapabilityConstraints::parse(arguments)?;

    let site = site.get_entity()?;
    let service = site.find_tosca_service()?;

    let mut index = 0;
    for node_id in service.get_contained_node_ids()? {
        let node = Entity::get(&node_id)?;
        if node.get_tosca_entity_kind()? == "NodeTemplate" {
            let (node_type_matches, node_template_matches, index_matches) = constraints.match_node(&node, index)?;

            if node_type_matches && node_template_matches && index_matches {
                for node_id in node.get_contained_node_ids()? {
                    let node = Entity::get(&node_id)?;
                    if node.get_tosca_entity_kind()? == "CapabilityAssignment" {
                        if constraints.match_capability(&node)? {
                            return Ok(node.id.to_string().into());
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
    capability_name: Option<&'own String>,

    node_type_id: Option<Id>,
    node_template: Option<(&'own String, usize)>,
}

impl<'own> CapabilityConstraints<'own> {
    fn parse(arguments: &'own Vec<Any>) -> Result<Self, String> {
        let mut find_node = Self::default();

        if let Some(argument) = arguments.first() {
            match argument {
                Any::NestedMap(argument) => {
                    let argument = argument.to_map();

                    if let Some(capability_type_name) = argument.into_get("capability_type_name") {
                        match capability_type_name {
                            Any::Text(capability_type_name) => {
                                find_node.capability_type_id = Some(Id::parse(Kind::Group, capability_type_name))
                            }

                            _ => return Err("capability_type_name is not a string".into()),
                        }
                    }

                    if let Some(capability_name) = argument.into_get("capability_name") {
                        match capability_name {
                            Any::Text(capability_name) => find_node.capability_name = Some(capability_name),

                            _ => return Err("capability_name is not a string".into()),
                        }
                    }

                    if let Some(node_type_name) = argument.into_get("node_type_name") {
                        match node_type_name {
                            Any::Text(node_type_name) => {
                                let mut id = Id::parse(Kind::Group, node_type_name);
                                id.prefix.insert(0, "node".into());
                                id.prefix.insert(0, "tosca".into());
                                find_node.node_type_id = Some(id)
                            }

                            _ => return Err("node_type_name is not a string".into()),
                        }
                    }

                    if let Some(node_template_name) = argument.into_get("node_template_name") {
                        match node_template_name {
                            Any::Text(node_template_name) => {
                                let node_template_index = argument
                                    .inner
                                    .get(&"node_template_index".into())
                                    .ok_or_else(|| "missing node_template_index")?;

                                let node_template_index = match node_template_index {
                                    Any::Integer(integer) => *integer as usize,
                                    Any::UnsignedInteger(unsigned_integer) => *unsigned_integer as usize,
                                    _ => return Err("node_template_index is not an integer".into()),
                                };

                                find_node.node_template = Some((node_template_name, node_template_index));
                            }

                            _ => return Err("node_template_name is not a string".into()),
                        }
                    }
                }

                _ => return Err("argument is not a map".into()),
            }
        }

        Ok(find_node)
    }

    fn match_capability(&self, entity: &Entity) -> Result<bool, String> {
        let mut capability_matches = match &self.capability_type_id {
            Some(capability_type_id) => entity.is_in_group(capability_type_id)?,
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
            Some(node_type_id) => entity.is_in_group(node_type_id)?,
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
