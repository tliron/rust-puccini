#![allow(unused)]

use super::tosca_instance_selector::*;

use floria_plugin_sdk::{data::*, traverse};

/// TOSCA entity.
pub trait ToscaEntity {
    /// Entity kind.
    fn get_tosca_entity_kind(&self) -> Result<String, String>;

    /// Is TOSCA entity.
    fn is_tosca_entity(&self) -> Result<bool, String> {
        self.get_tosca_entity_kind().map(|_| true)
    }

    /// Is TOSCA node.
    fn is_tosca_node(&self, node_template_name: &str) -> Result<bool, String>;

    /// Is TOSCA capability.
    fn is_tosca_capability(&self, capability_name: &str) -> Result<bool, String>;

    /// Is TOSCA requirement.
    fn is_tosca_requirement(&self, requirement_name: &str) -> Result<bool, String>;

    /// Capability.
    fn get_tosca_capability(&self, capability_name: &str) -> Result<Option<Entity>, String>;

    /// Outgoing relationship.
    fn get_tosca_outgoing_relationship(
        &self,
        requirement_name: &str,
        selector: ToscaInstanceSelector,
    ) -> Result<Option<Entity>, String>;

    /// Incoming relationship.
    fn get_tosca_incoming_relationship(
        &self,
        requirement_name: &str,
        selector: ToscaInstanceSelector,
    ) -> Result<Option<Entity>, String>;

    /// Source node.
    fn get_tosca_source_node(&self) -> Result<Entity, String>;

    /// Target capability.
    fn get_tosca_target_capability(&self) -> Result<Entity, String>;

    /// Target node.
    fn get_tosca_target_node(&self) -> Result<Entity, String> {
        let target_node = self.get_tosca_target_capability()?;
        let node = target_node
            .get_containing_node()?
            .ok_or_else(|| format!("TOSCA: capability {} is missing containing node", target_node.id))?;

        match node.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => Ok(node),

            kind => Err(format!(
                "TOSCA: container of relationship target capability {} is not a TOSCA node: {}",
                target_node.id, kind
            )),
        }
    }

    /// Service.
    fn find_tosca_service(&self) -> Result<Entity, String> {
        let node = Entity::get(&self.find_tosca_service_id()?)?;
        match node.get_tosca_entity_kind()?.as_str() {
            "ServiceTemplate" => Ok(node),

            kind => return Err(format!("TOSCA: entity {} is not a TOSCA service: {}", node.id, kind)),
        }
    }

    /// Service template ID.
    fn find_tosca_service_id(&self) -> Result<Id, String>;

    /// Node.
    fn find_tosca_node(
        &self,
        node_template_name: &str,
        _selector: ToscaInstanceSelector,
    ) -> Result<Option<Entity>, String> {
        let service = self.find_tosca_service()?;
        for contained_node_id in service.get_contained_node_ids()? {
            let node = Entity::get(&contained_node_id)?;
            if node.is_tosca_node(node_template_name)? {
                return Ok(Some(node));
            }
        }

        Ok(None)
    }
}

impl ToscaEntity for Entity {
    fn get_tosca_entity_kind(&self) -> Result<String, String> {
        let kind = traverse!(self.any, "metadata", "tosca:entity")
            .ok_or_else(|| format!("TOSCA: entity {} is not a TOSCA entity", self.id))?;

        match kind {
            Any::Text(kind) => Ok(kind.clone()),

            _ => Err(format!("TOSCA: entity {} has malformed \"tosca:entity\" metadata, not a string", self.id)),
        }
    }

    fn is_tosca_node(&self, node_template_name: &str) -> Result<bool, String> {
        if self.get_tosca_entity_kind()?.as_str() == "NodeTemplate" {
            let node_template_id = self
                .get_origin_template_id()?
                .ok_or_else(|| format!("TOSCA: node {} is missing origin_template_id", self.id))?;

            if node_template_id.id == node_template_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn is_tosca_capability(&self, capability_name: &str) -> Result<bool, String> {
        if self.get_tosca_entity_kind()? == "CapabilityAssignment" {
            let node_template_id = self
                .get_origin_template_id()?
                .ok_or_else(|| format!("TOSCA: capability {} is missing origin_template_id", self.id))?;

            if node_template_id.id == capability_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn is_tosca_requirement(&self, requirement_name: &str) -> Result<bool, String> {
        if self.get_tosca_entity_kind()? == "RequirementAssignment" {
            let relationship_template_id = self
                .get_origin_template_id()?
                .ok_or_else(|| format!("TOSCA: relationship {} is missing origin_template_id", self.id))?;

            let last_prefix_segment = relationship_template_id.prefix.last().ok_or_else(|| {
                format!("TOSCA: relationship {} has template {:?} with empty scope", self.id, relationship_template_id)
            })?;

            if last_prefix_segment == requirement_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn get_tosca_capability(&self, capability_name: &str) -> Result<Option<Entity>, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => {
                for contained_node_id in self.get_contained_node_ids()? {
                    let node = Entity::get(&contained_node_id)?;
                    if node.is_tosca_capability(capability_name)? {
                        return Ok(Some(node));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} is not a TOSCA node: {}", self.id, kind)),
        }

        Ok(None)
    }

    fn get_tosca_outgoing_relationship(
        &self,
        requirement_name: &str,
        _selector: ToscaInstanceSelector,
    ) -> Result<Option<Entity>, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => {
                for outgoing_relationship_id in self.get_outgoing_relationship_ids()? {
                    let relationship = Entity::get(&outgoing_relationship_id)?;
                    if relationship.is_tosca_requirement(requirement_name)? {
                        return Ok(Some(relationship));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} is not a TOSCA node: {}", self.id, kind)),
        }

        Ok(None)
    }

    fn get_tosca_incoming_relationship(
        &self,
        requirement_name: &str,
        _selector: ToscaInstanceSelector,
    ) -> Result<Option<Entity>, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "CapabilityAssignment" => {
                for incoming_relationship_id in self.get_incoming_relationship_ids()? {
                    let relationship = Entity::get(&incoming_relationship_id)?;
                    if relationship.is_tosca_requirement(requirement_name)? {
                        return Ok(Some(relationship));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} is not a capability: {}", self.id, kind)),
        }

        Ok(None)
    }

    fn get_tosca_source_node(&self) -> Result<Entity, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "RequirementAssignment" => {
                let node = self.get_source_node()?;
                match node.get_tosca_entity_kind()?.as_str() {
                    "NodeTemplate" => Ok(node),

                    kind => {
                        Err(format!("TOSCA: relationship {} source {} is not a TOSCA node: {}", self.id, node.id, kind))
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} is not a relationship: {}", self.id, kind)),
        }
    }

    fn get_tosca_target_capability(&self) -> Result<Entity, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "RequirementAssignment" => {
                let node = self.get_target_node()?;
                match node.get_tosca_entity_kind()?.as_str() {
                    "CapabilityAssignment" => Ok(node),

                    kind => Err(format!(
                        "TOSCA: relationship {} target {} is not a TOSCA capability: {}",
                        self.id, node.id, kind
                    )),
                }
            }

            kind => return Err(format!("TOSCA: entity {} is not a TOSCA relationship: {}", self.id, kind)),
        }
    }

    fn find_tosca_service_id(&self) -> Result<Id, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => self
                .get_containing_node_id()?
                .ok_or_else(|| format!("TOSCA: node {} is missing containing node", self.id)),
            "CapabilityAssignment" => {
                let node = self
                    .get_containing_node()?
                    .ok_or_else(|| format!("TOSCA: capability {} is missing containing node", self.id))?;
                node.find_tosca_service_id()
            }

            "RequirementAssignment" => self.get_source_node()?.find_tosca_service_id(),

            kind => return Err(format!("TOSCA: entity {} is incompatible: {}", self.id, kind)),
        }
    }
}
