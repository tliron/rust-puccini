use super::{
    super::{entity::*, errors::*, name::*, utils::*},
    source::*,
};

use {kutil::std::immutable::*, std::collections::*};

impl Source {
    /// Entity names.
    pub fn entity_names(&self) -> Vec<(EntityKind, Name)> {
        self.entities.keys().map(|key| (key.entity_kind, key.inner.clone())).collect()
    }

    /// Entitiy names as a sorted tree.
    pub fn entity_names_tree(&self) -> BTreeMap<EntityKind, Vec<Name>> {
        let mut entity_names_tree = BTreeMap::<EntityKind, Vec<Name>>::default();

        for key in self.entities.keys() {
            match entity_names_tree.get_mut(&key.entity_kind) {
                Some(names) => {
                    names.push(key.inner.clone());
                    names.sort();
                }

                None => {
                    let names = vec![key.inner.clone()];
                    entity_names_tree.insert(key.entity_kind, names);
                }
            }
        }

        entity_names_tree
    }

    /// Whether we have this entity.
    pub fn has_entity(&self, entity_kind: EntityKind, full_name: &FullName) -> bool {
        full_name.scope.is_empty()
            && self.entities.contains_key(&WithEntityKind::new(entity_kind, full_name.name.clone()))
    }

    /// Whether all entities are complete.
    pub fn are_entities_complete(&self) -> bool {
        self.entities.values().all(|entity| entity.is_complete())
    }

    /// Add an [EntityRef].
    ///
    /// This will also add entity's name in the empty scope.
    pub fn add_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = self.source_id.to_string(), "adding entity: {}", name);
        match self.entities.insert(WithEntityKind::new(entity_kind, name.clone()), entity) {
            Some(_) => Err(NameReusedError::new(name.to_string())),
            None => {
                self.namespace.insert(WithEntityKind::new(entity_kind, name.into()), self.source_id.clone());
                Ok(())
            }
        }
    }

    /// Add an [Entity].
    pub fn add_entity<EntityT, AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        name: Name,
        entity: EntityT,
    ) -> Result<(), NameReusedError<AnnotatedT>>
    where
        EntityT: 'static + Entity,
        AnnotatedT: Default,
    {
        self.add_entity_ref(entity_kind, name.clone(), entity.into())
    }

    /// Get an [EntityRef].
    pub fn get_entity_ref<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<&EntityRef, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.entities
            .get(&WithEntityKind::new(entity_kind, name.clone()))
            .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), name.to_string()))
    }

    /// Get an [Entity].
    pub fn get_entity<EntityT, AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<&EntityT, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Default,
    {
        let entity = self.get_entity_ref(entity_kind, entity_kind_name, name)?;
        Ok(entity.downcast_ref_or_error(name.to_string())?)
    }

    /// Remove an entity and return its [EntityRef] if it exists.
    ///
    /// Note that this will *not* remove the entity's name.
    pub fn remove_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        name: &Name,
    ) -> Result<EntityRef, UndeclaredError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(
            source = self.source_id.to_string(),
            kind = entity_kind_name.to_string(),
            "removing entity: {}",
            name
        );

        self.entities
            .remove(&WithEntityKind::new(entity_kind, name.clone()))
            .ok_or_else(|| UndeclaredError::new(entity_kind_name.to_string(), name.to_string()))
    }
}
