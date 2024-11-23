use super::super::super::super::data::*;

use {
    floria_plugin_sdk::data::*,
    std::{iter::*, slice::*},
};

//
// ToscaPathParser
//

/// TOSCA Path parser.
pub struct ToscaPathParser<'own> {
    iterator: Peekable<Iter<'own, Expression>>,
}

impl<'own> ToscaPathParser<'own> {
    /// Constructor.
    pub fn new(arguments: &'own Vec<Expression>) -> Self {
        Self { iterator: arguments.iter().peekable() }
    }

    /// Next site.
    pub fn next_site(&mut self, path_site: Entity) -> Result<Entity, String> {
        let argument = self.iterator.next().ok_or_else(|| "invalid TOSCA path: empty")?;
        match argument {
            Expression::Text(text) => match text.as_str() {
                // SELF, <node_context> |
                // SELF, <rel_context>
                "SELF" => match path_site.id.kind {
                    Kind::Vertex => Ok(self.next_site_after_node(&path_site)?.unwrap_or_else(|| path_site)),

                    Kind::Edge => Ok(self.next_site_after_relationship(&path_site)?.unwrap_or_else(|| path_site)),

                    kind => Err(format!("TOSCA path: entity not a node or a relationship: {}", kind)),
                },

                // <node_symbolic_name>, <idx>, <node_context> |
                // TODO? <relationship_symbolic_name>, <rel_context>
                name => {
                    let selector = self.next_selector()?;
                    let node = path_site
                        .find_tosca_node(name, selector.clone())?
                        .ok_or_else(|| format!("TOSCA path: node not found: {} {}", name, selector))?;

                    Ok(self.next_site_after_node(&node)?.unwrap_or_else(|| path_site))
                }
            },

            _ => Err(format!("invalid TOSCA path: argument not a string: {}", argument.type_name())),
        }
    }

    /// Next site after node.
    pub fn next_site_after_node(&mut self, path_site: &Entity) -> Result<Option<Entity>, String> {
        match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // RELATIONSHIP, <requirement_name>, <idx>, <rel_context>
                    "RELATIONSHIP" => {
                        self.iterator.next();
                        let requirement_name =
                            self.iterator.next().ok_or_else(|| "invalid TOSCA path: missing requirement name")?;

                        match requirement_name {
                            Expression::Text(requirement_name) => {
                                let selector = self.next_selector()?;
                                let relationship = path_site
                                    .get_tosca_outgoing_relationship(requirement_name, selector.clone())?
                                    .ok_or_else(|| {
                                        format!("TOSCA path: requirement not found: {} {}", requirement_name, selector)
                                    })?;

                                Ok(Some(match self.next_site_after_relationship(&relationship)? {
                                    Some(site) => site,
                                    None => relationship,
                                }))
                            }

                            _ => Err(format!(
                                "invalid TOSCA path: requirement name not a string: {}",
                                requirement_name.type_name()
                            )),
                        }
                    }

                    // CAPABILITY, <capability_name>, RELATIONSHIP, <idx>, <rel_context> |
                    // CAPABILITY, <capability_name>
                    "CAPABILITY" => {
                        self.iterator.next();
                        let capability_name =
                            self.iterator.next().ok_or_else(|| "invalid TOSCA path: missing capability name")?;

                        match capability_name {
                            Expression::Text(capability_name) => {
                                let capability_node = path_site
                                    .get_tosca_capability(capability_name)?
                                    .ok_or_else(|| format!("TOSCA path: capability not found: {}", capability_name))?;

                                Ok(Some(match self.next_site_after_capability(&capability_node)? {
                                    Some(site) => site,
                                    None => capability_node,
                                }))
                            }

                            _ => Err(format!(
                                "invalid TOSCA path: capability name not a string: {}",
                                capability_name.type_name()
                            )),
                        }
                    }

                    _ => Ok(None),
                },

                _ => Ok(None),
            },

            None => Ok(None),
        }
    }

    /// Next site after relationship.
    pub fn next_site_after_relationship(&mut self, path_site: &Entity) -> Result<Option<Entity>, String> {
        Ok(match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // SOURCE, <node_context>
                    "SOURCE" => {
                        self.iterator.next();
                        let node = path_site.get_tosca_source_node()?;
                        Some(match self.next_site_after_node(&node)? {
                            Some(site) => site,
                            None => node,
                        })
                    }

                    // TARGET, <node_context>
                    "TARGET" => {
                        self.iterator.next();
                        let node = path_site.get_tosca_target_node()?;
                        Some(match self.next_site_after_node(&node)? {
                            Some(site) => site,
                            None => node,
                        })
                    }

                    // CAPABILITY, RELATIONSHIP <idx>, <rel_context> | CAPABILITY
                    "CAPABILITY" => {
                        self.iterator.next();
                        let node = path_site.get_tosca_target_capability()?;
                        Some(match self.next_site_after_capability(&node)? {
                            Some(site) => site,
                            None => node,
                        })
                    }

                    _ => None,
                },

                _ => None,
            },

            None => None,
        })
    }

    /// Next site after capability.
    pub fn next_site_after_capability(&mut self, path_site: &Entity) -> Result<Option<Entity>, String> {
        match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // RELATIONSHIP <idx>, <rel_context>
                    "RELATIONSHIP" => {
                        // Broken! See: https://github.com/oasis-tcs/tosca-specs/issues/315
                        self.iterator.next();
                        let selector = self.next_selector()?;
                        let requirement_name = "broken";
                        let relationship =
                            path_site.get_tosca_incoming_relationship(requirement_name, selector.clone())?.ok_or_else(
                                || format!("TOSCA path: requirement not found: {} {}", requirement_name, selector),
                            )?;

                        Ok(Some(match self.next_site_after_relationship(&relationship)? {
                            Some(site) => site,
                            None => relationship,
                        }))
                    }

                    _ => Ok(None),
                },

                _ => Ok(None),
            },

            None => Ok(None),
        }
    }

    /// Next selector for node or relationship.
    pub fn next_selector(&mut self) -> Result<ToscaInstanceSelector, String> {
        Ok(match self.iterator.peek() {
            Some(index) => match index {
                // ALL
                Expression::Text(text) => match text.as_str() {
                    "ALL" => {
                        self.iterator.next();
                        ToscaInstanceSelector::All
                    }

                    _ => Default::default(),
                },

                // <integer_index>
                Expression::Integer(integer) => {
                    self.iterator.next();
                    ToscaInstanceSelector::Index(*integer as usize)
                }

                // <integer_index>
                Expression::UnsignedInteger(unsigned_integer) => {
                    self.iterator.next();
                    ToscaInstanceSelector::Index(*unsigned_integer as usize)
                }

                _ => Default::default(),
            },

            None => Default::default(),
        })
    }

    /// Next property.
    pub fn next_property<'site>(
        &mut self,
        path_site: &'site Entity,
        read_only: bool,
    ) -> Result<Property<'site>, String> {
        let argument = self
            .iterator
            .next()
            .ok_or_else(|| format!("invalid TOSCA path: missing {} name", property_or_attribute(read_only)))?;

        match argument {
            Expression::Text(property_name) => {
                let property = path_site.get_property(property_name)?.ok_or_else(|| {
                    format!("TOSCA path: {} {} not found", property_or_attribute(read_only), argument.to_string())
                })?;

                if property.is_read_only()? == read_only {
                    Ok(property)
                } else {
                    Err(format!(
                        "TOSCA path: {} {} found but {}read only",
                        property_or_attribute(read_only),
                        argument.to_string(),
                        if read_only { "" } else { "not " },
                    ))
                }
            }

            _ => Err(format!(
                "invalid TOSCA path: {} name not a string: {}",
                property_or_attribute(read_only),
                argument.type_name()
            )),
        }
    }

    /// Next expression.
    pub fn next_expression(&mut self, property: &Property) -> Result<Expression, String> {
        let Some(mut current_value) = property.value() else {
            return Err(format!("TOSCA path: no value for property: {}", property.name));
        };

        while let Some(argument) = self.iterator.next() {
            let value = current_value
                .get(argument)
                .ok_or_else(|| format!("TOSCA path: {} not found in value", argument.to_string()))?;

            current_value = value;
        }

        Ok(current_value.clone())
    }
}

fn property_or_attribute(read_only: bool) -> &'static str {
    match read_only {
        true => "attribute",
        false => "property",
    }
}
