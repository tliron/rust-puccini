use super::{
    super::{entity::*, errors::*, name::*, source::*},
    catalog::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl Catalog {
    /// Whether all entities are complete.
    pub fn are_entities_complete(&self) -> bool {
        self.sources.values().all(|source| source.are_entities_complete())
    }

    /// Complete all entities.
    ///
    /// Calls [complete](super::super::entity::Entity::complete) on entities for which
    /// [should_complete](super::super::entity::Entity::should_complete) is true.
    ///
    /// Note that each entity is removed from the catalog while it is being completed.
    pub fn complete_entities<AnnotatedT, ErrorRecipientT>(
        &mut self,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut entities = self.entity_names();
        entities.sort();

        for (source_id, entity_kind, name) in entities {
            let entity_kinds = unwrap_or_give!(self.source_entity_kinds(&source_id), errors, &EntityKinds::default());
            let entity_kind_name = entity_kinds.represent(entity_kind);

            match self.sources.get_mut(&source_id) {
                Some(source) => {
                    match source.remove_entity_ref::<WithoutAnnotations>(entity_kind, &entity_kind_name, &name) {
                        Ok(mut entity) => {
                            self.complete_entity(
                                &mut entity,
                                &entity_kind_name,
                                &name,
                                &source_id,
                                &mut CallStack::new(source_id.clone(), name.clone()),
                                errors,
                            )?;

                            match self.sources.get_mut(&source_id) {
                                Some(source) => {
                                    unwrap_or_give!(source.add_entity_ref(entity_kind, name, entity), errors)
                                }

                                None => panic!("source {} disappeared", source_id),
                            }
                        }

                        Err(_) => {
                            panic!("{} {} disappeared from {}", entity_kind_name, name, source_id);
                        }
                    }
                }

                None => {
                    panic!("source {} disappeared", source_id);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn complete_entity<AnnotatedT, ErrorRecipientT>(
        &mut self,
        entity: &mut EntityRef,
        entity_kind_name: &ByteString,
        name: &Name,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(if entity.should_complete() {
            tracing::debug!(
                source = source_id.to_string(),
                kind = entity_kind_name.to_string(),
                name = name.to_string(),
                "completing",
            );

            entity
                .complete(self, source_id, callstack, errors.into_annotated().to_ref())
                .map_err(|error| error.into_annotated())?;

            if entity.is_complete() {
                tracing::debug!(
                    source = source_id.to_string(),
                    kind = entity_kind_name.to_string(),
                    name = name.to_string(),
                    "completed",
                );
                true
            } else {
                tracing::error!(
                    source = source_id.to_string(),
                    kind = entity_kind_name.to_string(),
                    name = name.to_string(),
                    "could not complete",
                );
                false
            }
        } else {
            tracing::trace!(
                source = source_id.to_string(),
                kind = entity_kind_name.to_string(),
                name = name.to_string(),
                "should not complete",
            );
            entity.is_complete()
        })
    }
}
