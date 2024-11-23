use {
    kutil::std::{collections::*, immutable::*},
    std::fmt,
};

//
// EntityKind
//

/// Entity kind.
///
/// Their names are provided by
/// [Dialect::entity_kind_name](super::super::dialect::Dialect::entity_kind_name).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityKind(pub usize);

impl fmt::Display for EntityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl From<usize> for EntityKind {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Into<usize> for EntityKind {
    fn into(self) -> usize {
        self.0
    }
}

//
// WithEntityKind
//

/// With [EntityKind].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithEntityKind<InnerT> {
    /// Entity kind.
    pub entity_kind: EntityKind,

    /// Inner.
    pub inner: InnerT,
}

impl<InnerT> WithEntityKind<InnerT> {
    /// Constructor.
    pub fn new(entity_kind: EntityKind, inner: InnerT) -> Self {
        Self { entity_kind, inner }
    }
}

//
// EntityKinds
//

/// Map of [EntityKind].
#[derive(Clone, Debug, Default)]
pub struct EntityKinds(pub FastHashMap<EntityKind, ByteString>);

impl EntityKinds {
    /// Add support for an entity kind.
    pub fn add(&mut self, entity_kind: EntityKind, name: ByteString) {
        self.0.insert(entity_kind, name);
    }

    /// Name of an entity kind, if supported.
    pub fn get_name(&self, entity_kind: EntityKind) -> Option<&ByteString> {
        self.0.get(&entity_kind)
    }

    /// Representation of an entity kind.
    ///
    /// Will use its name if supported, otherwise will use a numeric representation.
    pub fn represent(&self, entity_kind: EntityKind) -> ByteString {
        self.get_name(entity_kind).cloned().unwrap_or_else(|| format!("EntityKind#{}", entity_kind).into())
    }
}
