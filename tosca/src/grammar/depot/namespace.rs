use super::{
    super::{entity::*, errors::*, name::*, source::*},
    depot::*,
};

use kutil::std::zerocopy::*;

impl Depot {
    /// Find the [SourceID] of a [FullName].
    pub fn try_lookup<'own>(
        &'own self,
        entity_kind: EntityKind,
        full_name: &FullName,
        source_id: &'own SourceID,
    ) -> Option<&'own SourceID> {
        self.sources.get(source_id)?.try_lookup(entity_kind, full_name)
    }

    /// Find the [Source] of an entity.
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

    /// Find the [Source] of an entity.
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
