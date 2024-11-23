use super::{
    super::{entity::*, errors::*, name::*, source::*},
    catalog::*,
};

use kutil::std::immutable::*;

impl Catalog {
    /// Find the source ID of a full name.
    pub fn try_lookup<'own>(
        &'own self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &'own SourceID,
    ) -> Option<&'own SourceID> {
        self.sources.get(source_id)?.try_lookup(entity_kind, full_name)
    }

    /// Find the source of an entity.
    pub fn lookup<AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        source_id: &SourceID,
        full_name: &FullName,
    ) -> Result<&Source, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let source = self.get_source(source_id)?;
        let entity_source_id = source.lookup(entity_kind, entity_kind_name, full_name)?;
        Ok(self.get_source(entity_source_id)?)
    }

    /// Find the source of an entity.
    pub fn lookup_mut<AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &ByteString,
        source_id: &SourceID,
        full_name: &FullName,
    ) -> Result<&mut Source, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        let source = self.get_source(source_id)?;
        let entity_source_id = source.lookup(entity_kind, entity_kind_name, full_name)?.clone();
        Ok(self.get_source_mut(&entity_source_id)?)
    }
}
