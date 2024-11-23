use super::{
    super::{entity::*, errors::*, name::*, source::*, utils::*},
    catalog::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl Catalog {
    /// All entity names.
    pub fn entity_names(&self) -> Vec<(SourceID, EntityKind, Name)> {
        let mut entity_names = Vec::default();
        for (source_id, source) in &self.sources {
            entity_names.extend(
                source.entity_names().into_iter().map(|(entity_kind, name)| (source_id.clone(), entity_kind, name)),
            );
        }
        entity_names
    }

    /// Add an entity reference.
    pub fn add_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        source_id: &SourceID,
        name: Name,
        entity: EntityRef,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(self.get_source_mut(source_id)?.add_entity_ref(entity_kind, name, entity)?)
    }

    /// Get an entity reference.
    pub fn get_entity_ref<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<&EntityRef, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup(entity_kind, &entity_kind_name, source_id, full_name)?;
        Ok(source.get_entity_ref(entity_kind, &entity_kind_name, &full_name.name)?)
    }

    /// Get an entity reference,
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    pub fn get_complete_entity_ref<AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityRef>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let source = unwrap_or_give_and_return!(self.get_source(source_id), errors, Ok(None));

        let entity_kind_name =
            unwrap_or_give_and_return!(self.dialect_entity_kinds(&source.dialect_id), errors, Ok(None))
                .represent(entity_kind);

        let entity_source_id =
            unwrap_or_give_and_return!(source.lookup(entity_kind, &entity_kind_name, full_name), errors, Ok(None))
                .clone();

        unwrap_or_give_and_return!(callstack.add(entity_source_id.clone(), full_name.name.clone()), errors, Ok(None));

        let entity_source = unwrap_or_give_and_return!(self.get_source_mut(&entity_source_id), errors, Ok(None));

        let mut entity = unwrap_or_give_and_return!(
            entity_source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name),
            errors,
            Ok(None)
        );

        let complete = {
            match self.complete_entity(
                &mut entity,
                &entity_kind_name,
                &full_name.name,
                &entity_source_id,
                callstack,
                errors,
            ) {
                Ok(complete) => complete,
                Err(error) => {
                    if self
                        .add_entity_ref::<WithoutAnnotations>(
                            entity_kind,
                            &entity_source_id,
                            full_name.name.clone(),
                            entity,
                        )
                        .is_err()
                    {
                        panic!("source {} disappeared", &entity_source_id);
                    }
                    return Err(error);
                }
            }
        };

        if self
            .add_entity_ref::<WithoutAnnotations>(entity_kind, &entity_source_id, full_name.name.clone(), entity)
            .is_err()
        {
            panic!("source {} disappeared", &entity_source_id);
        }

        Ok(if complete {
            match self.get_entity_ref::<WithoutAnnotations>(entity_kind, full_name, source_id) {
                Ok(entity) => Some(entity),

                Err(_) => {
                    panic!("{} {} disappeared from {}", entity_kind_name, full_name, entity_source_id);
                }
            }
        } else {
            None
        })
    }

    /// Get an [Entity].
    pub fn get_entity<EntityT, AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<&EntityT, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Default,
    {
        let entity = self.get_entity_ref(entity_kind, full_name, source_id)?;
        Ok(entity.downcast_ref_or_error(full_name.to_string())?)
    }

    /// Get an [Entity],
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// The call is added to the callstack in order to detect circular dependencies.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    pub fn get_complete_entity_next<EntityT, AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityT>, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(match self.get_complete_entity_ref(entity_kind, full_name, source_id, callstack, errors)? {
            Some(entity) => match entity.downcast_ref_or_error(full_name.to_string()) {
                Ok(entity) => Some(entity),
                Err(error) => {
                    errors.give(error)?;
                    None
                }
            },
            None => None,
        })
    }

    /// Get an [Entity],
    /// calling [complete](Entity::complete) on it if
    /// [should_complete](Entity::should_complete) is true.
    ///
    /// A [CallStack] is created in order to detect circular dependencies.
    ///
    /// Note that the entity is removed from the catalog while it is being completed.
    pub fn get_complete_entity<EntityT, AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&EntityT>, ToscaError<AnnotatedT>>
    where
        EntityT: 'static,
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        self.get_complete_entity_next(entity_kind, full_name, source_id, &mut Default::default(), errors)
    }

    /// Remove an entity and return its entity reference and source ID if it exists.
    pub fn remove_entity_ref<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(EntityRef, SourceID), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let entity_kind_name = self.source_entity_kinds(source_id)?.represent(entity_kind);
        let source = self.lookup_mut(entity_kind, &entity_kind_name, source_id, full_name)?;
        let entity = source.remove_entity_ref(entity_kind, &entity_kind_name, &full_name.name)?;
        Ok((entity, source.source_id.clone()))
    }
}
