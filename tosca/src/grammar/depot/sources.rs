use super::{
    super::{entity::*, errors::*, source::*},
    depot::*,
};

impl Depot {
    /// Add a [Source].
    pub fn add_source(&mut self, source: Source) {
        self.sources.insert(source.source_id.clone(), source);
    }

    /// Get a [Source].
    pub fn get_source<AnnotatedT>(&self, source_id: &SourceID) -> Result<&Source, SourceNotLoadedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.sources.get(source_id).ok_or_else(|| SourceNotLoadedError::new(source_id.clone()))
    }

    /// Get a [Source].
    pub fn get_source_mut<AnnotatedT>(
        &mut self,
        source_id: &SourceID,
    ) -> Result<&mut Source, SourceNotLoadedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.sources.get_mut(source_id).ok_or_else(|| SourceNotLoadedError::new(source_id.clone()))
    }

    /// Supported [EntityKind]s.
    pub fn source_entity_kinds<AnnotatedT>(&self, source_id: &SourceID) -> Result<&EntityKinds, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(self.dialect_entity_kinds(&self.get_source(source_id)?.dialect_id)?)
    }
}
