use super::{
    super::{dialect::*, entity::*, name::*},
    id::*,
};

use kutil::std::collections::*;

//
// Source
//

/// Source.
#[derive(Debug)]
pub struct Source {
    /// Source ID.
    pub source_id: SourceID,

    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Dependencies.
    pub dependencies: FastHashMap<SourceID, Scope>,

    /// Entities.
    pub entities: FastHashMap<WithEntityKind<Name>, EntityRef>,

    /// Namespace.
    pub namespace: FastHashMap<WithEntityKind<FullName>, SourceID>,
}

impl Source {
    /// Constructor.
    pub fn new(source_id: SourceID, dialect_id: DialectID) -> Self {
        Self {
            source_id,
            dialect_id,
            dependencies: Default::default(),
            entities: Default::default(),
            namespace: Default::default(),
        }
    }

    /// Add a dependency.
    pub fn add_dependency(&mut self, source_id: SourceID, scope: Scope) {
        tracing::trace!(source = self.source_id.to_string(), "adding dependency: {} -> {}", source_id, scope);
        self.dependencies.insert(source_id, scope);
    }
}
