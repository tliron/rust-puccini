use super::tosca_instance_selector::*;

use floria_plugin_sdk::{data::*, traverse};

//
// ToscaEntity
//

/// TOSCA entity.
pub trait ToscaEntity {
    /// Entity kind.
    fn get_tosca_entity_kind(&self) -> Result<String, String>;

    /// Is TOSCA entity.
    fn _is_tosca_entity(&self) -> Result<bool, String> {
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
        let target_vertex = self.get_tosca_target_capability()?;
        let vertex = target_vertex
            .get_containing_vertex()?
            .ok_or_else(|| format!("TOSCA: capability {} missing `containing node`", target_vertex.id))?;

        match vertex.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => Ok(vertex),

            kind => Err(format!(
                "TOSCA: container of relationship target capability {} not a TOSCA node: {}",
                target_vertex.id, kind
            )),
        }
    }

    /// Service.
    fn find_tosca_service(&self) -> Result<Entity, String> {
        let node = Entity::get(&self.find_tosca_service_id()?)?;
        match node.get_tosca_entity_kind()?.as_str() {
            "ServiceTemplate" => Ok(node),

            kind => return Err(format!("TOSCA: entity {} not a TOSCA service: {}", node.id, kind)),
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
        for contained_vertex_id in service.get_contained_vertex_ids()? {
            let vertex = Entity::get(&contained_vertex_id)?;
            if vertex.is_tosca_node(node_template_name)? {
                return Ok(Some(vertex));
            }
        }

        Ok(None)
    }
}

impl ToscaEntity for Entity {
    fn get_tosca_entity_kind(&self) -> Result<String, String> {
        let kind = traverse!(self.expression, "metadata", "tosca:entity")
            .ok_or_else(|| format!("TOSCA: entity {} not a TOSCA entity", self.id))?;

        match kind {
            Expression::Text(kind) => Ok(kind.clone()),

            _ => Err(format!("TOSCA: entity {} has malformed \"tosca:entity\" metadata, not a string", self.id)),
        }
    }

    fn is_tosca_node(&self, node_template_name: &str) -> Result<bool, String> {
        if self.get_tosca_entity_kind()?.as_str() == "NodeTemplate" {
            let node_template_id = self
                .get_origin_template_id()?
                .ok_or_else(|| format!("TOSCA: node {} missing \"origin_template_id\"", self.id))?;

            return Ok(node_template_id.id == node_template_name);
        }

        Ok(false)
    }

    fn is_tosca_capability(&self, capability_name: &str) -> Result<bool, String> {
        if self.get_tosca_entity_kind()? == "CapabilityAssignment" {
            let node_template_id = self
                .get_origin_template_id()?
                .ok_or_else(|| format!("TOSCA: capability {} missing \"origin_template_id\"", self.id))?;

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
                .ok_or_else(|| format!("TOSCA: relationship {} missing `origin_template_id`", self.id))?;

            let last_directory_segment = relationship_template_id.directory.last().ok_or_else(|| {
                format!(
                    "TOSCA: relationship {} has template {:?} with empty directory",
                    self.id, relationship_template_id
                )
            })?;

            return Ok(last_directory_segment == requirement_name);
        }

        Ok(false)
    }

    fn get_tosca_capability(&self, capability_name: &str) -> Result<Option<Entity>, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => {
                for contained_vertex_id in self.get_contained_vertex_ids()? {
                    let vertex = Entity::get(&contained_vertex_id)?;
                    if vertex.is_tosca_capability(capability_name)? {
                        return Ok(Some(vertex));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} not a TOSCA node: {}", self.id, kind)),
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
                for outgoing_edge_id in self.get_outgoing_edge_ids()? {
                    let edge = Entity::get(&outgoing_edge_id)?;
                    if edge.is_tosca_requirement(requirement_name)? {
                        return Ok(Some(edge));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} not a TOSCA node: {}", self.id, kind)),
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
                for incoming_edge_id in self.get_incoming_edge_ids()? {
                    let relationship = Entity::get(&incoming_edge_id)?;
                    if relationship.is_tosca_requirement(requirement_name)? {
                        return Ok(Some(relationship));
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} not a capability: {}", self.id, kind)),
        }

        Ok(None)
    }

    fn get_tosca_source_node(&self) -> Result<Entity, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "RequirementAssignment" => {
                let vertex = self.get_source_vertex()?;
                match vertex.get_tosca_entity_kind()?.as_str() {
                    "NodeTemplate" => Ok(vertex),

                    kind => {
                        Err(format!("TOSCA: relationship {} source {} not a TOSCA node: {}", self.id, vertex.id, kind))
                    }
                }
            }

            kind => return Err(format!("TOSCA: entity {} not a relationship: {}", self.id, kind)),
        }
    }

    fn get_tosca_target_capability(&self) -> Result<Entity, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "RequirementAssignment" => {
                let vertex = self.get_target_vertex()?;
                match vertex.get_tosca_entity_kind()?.as_str() {
                    "CapabilityAssignment" => Ok(vertex),

                    kind => Err(format!(
                        "TOSCA: relationship {} target {} not a TOSCA capability: {}",
                        self.id, vertex.id, kind
                    )),
                }
            }

            kind => return Err(format!("TOSCA: entity {} not a TOSCA relationship: {}", self.id, kind)),
        }
    }

    fn find_tosca_service_id(&self) -> Result<Id, String> {
        match self.get_tosca_entity_kind()?.as_str() {
            "NodeTemplate" => self
                .get_containing_vertex_id()?
                .ok_or_else(|| format!("TOSCA: node {} missing containing node", self.id)),
            "CapabilityAssignment" => {
                let node = self
                    .get_containing_vertex()?
                    .ok_or_else(|| format!("TOSCA: capability {} missing containing node", self.id))?;
                node.find_tosca_service_id()
            }

            "RequirementAssignment" => self.get_source_vertex()?.find_tosca_service_id(),

            kind => return Err(format!("TOSCA: entity {} incompatible: {}", self.id, kind)),
        }
    }
}
