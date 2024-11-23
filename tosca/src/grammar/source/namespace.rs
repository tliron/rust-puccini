use super::{
    super::{entity::*, errors::*, name::*},
    id::*,
    source::*,
};

use {kutil::std::immutable::*, std::collections::*};

impl Source {
    /// Namespace.
    pub fn namespace(&self) -> Vec<(EntityKind, FullName, SourceID)> {
        self.namespace.iter().map(|(key, source_id)| (key.entity_kind, key.inner.clone(), source_id.clone())).collect()
    }

    /// Namespace as a sorted tree.
    pub fn namespace_tree(&self) -> BTreeMap<EntityKind, BTreeMap<FullName, SourceID>> {
        let mut namespace = BTreeMap::<EntityKind, BTreeMap<FullName, SourceID>>::default();

        for (entity_kind, full_name, source_id) in self.namespace() {
            match namespace.get_mut(&entity_kind) {
                Some(names) => {
                    names.insert(full_name, source_id);
                }

                None => {
                    let mut names = BTreeMap::default();
                    names.insert(full_name, source_id);
                    namespace.insert(entity_kind, names);
                }
            }
        }

        namespace
    }

    /// Map a [FullName] to a [SourceID].
    pub fn map_name<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: FullName,
        source_id: SourceID,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = self.source_id.to_string(), "adding imported entity: {} -> {}", full_name, source_id);

        match self.namespace.insert(WithEntityKind::new(entity_kind, full_name.clone()), source_id) {
            Some(_) => Err(NameReusedError::new(full_name.to_string())),
            None => Ok(()),
        }
    }

    /// Find the [SourceID] of a [FullName].
    pub fn try_lookup(&self, entity_kind: EntityKind, full_name: &FullName) -> Option<&SourceID> {
        self.namespace.get(&WithEntityKind::new(entity_kind, full_name.clone()))
    }

    /// Find the [SourceID] of a [FullName].
    pub fn lookup<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        full_name: &FullName,
    ) -> Result<&SourceID, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.try_lookup(entity_kind, full_name)
            .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), full_name.to_string()))
    }
}
