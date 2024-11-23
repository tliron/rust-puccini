use super::super::super::super::grammar::*;

// Types

/// Artifact type.
pub const ARTIFACT_TYPE: EntityKind = EntityKind(1);

/// Capability type.
pub const CAPABILITY_TYPE: EntityKind = EntityKind(2);

/// Data type.
pub const DATA_TYPE: EntityKind = EntityKind(3);

/// Group type.
pub const GROUP_TYPE: EntityKind = EntityKind(4);

/// Interface type.
pub const INTERFACE_TYPE: EntityKind = EntityKind(5);

/// Node type.
pub const NODE_TYPE: EntityKind = EntityKind(6);

/// Policy type.
pub const POLICY_TYPE: EntityKind = EntityKind(7);

/// Relationship type.
pub const RELATIONSHIP_TYPE: EntityKind = EntityKind(8);

// Templates

/// Group template.
pub const GROUP_TEMPLATE: EntityKind = EntityKind(104);

/// Node template.
pub const NODE_TEMPLATE: EntityKind = EntityKind(106);

/// Policy template.
pub const POLICY_TEMPLATE: EntityKind = EntityKind(107);

/// Relationship template.
pub const RELATIONSHIP_TEMPLATE: EntityKind = EntityKind(108);

// Other

/// Repository.
pub const REPOSITORY: EntityKind = EntityKind(200);

/// TOSCA 2.0 supported [EntityKind]s.
pub fn entity_kinds() -> EntityKinds {
    let mut entity_kinds = EntityKinds::default();

    entity_kinds.add(ARTIFACT_TYPE, "ArtifactType".into());
    entity_kinds.add(CAPABILITY_TYPE, "CapabilityType".into());
    entity_kinds.add(DATA_TYPE, "DataType".into());
    entity_kinds.add(GROUP_TYPE, "GroupType".into());
    entity_kinds.add(INTERFACE_TYPE, "InterfaceType".into());
    entity_kinds.add(NODE_TYPE, "NodeType".into());
    entity_kinds.add(POLICY_TYPE, "PolicyType".into());
    entity_kinds.add(RELATIONSHIP_TYPE, "RelationshipType".into());

    entity_kinds.add(GROUP_TEMPLATE, "GroupTemplate".into());
    entity_kinds.add(NODE_TEMPLATE, "NodeTemplate".into());
    entity_kinds.add(POLICY_TEMPLATE, "PolicyTemplate".into());
    entity_kinds.add(RELATIONSHIP_TEMPLATE, "RelationshipTemplate".into());

    entity_kinds.add(REPOSITORY, "Repository".into());

    entity_kinds
}
