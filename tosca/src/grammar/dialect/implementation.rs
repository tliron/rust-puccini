use super::{super::entity::*, id::*};

//
// DialectImplementation
//

/// Dialect implementation.
#[derive(Clone, Debug)]
pub struct DialectImplementation {
    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Supported entity kinds.
    pub entity_kinds: EntityKinds,
}

impl DialectImplementation {
    /// Constructor.
    pub fn new(dialect_id: DialectID, entity_kinds: EntityKinds) -> Self {
        Self { dialect_id, entity_kinds }
    }

    /// Dialect ID.
    pub fn dialect_id(&self) -> DialectID {
        self.dialect_id.clone()
    }
}
