use super::{
    super::{entity::*, errors::*, source::*},
    catalog::*,
};

impl Catalog {
    /// Add a source.
    pub fn add_source(&mut self, source: Source) {
        self.sources.insert(source.source_id.clone(), source);
    }

    /// Get a source.
    pub fn get_source<AnnotatedT>(&self, source_id: &SourceID) -> Result<&Source, SourceNotLoadedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.sources.get(source_id).ok_or_else(|| SourceNotLoadedError::new(source_id.clone()))
    }

    /// Get a source.
    pub fn get_source_mut<AnnotatedT>(
        &mut self,
        source_id: &SourceID,
    ) -> Result<&mut Source, SourceNotLoadedError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        self.sources.get_mut(source_id).ok_or_else(|| SourceNotLoadedError::new(source_id.clone()))
    }

    /// Supported entity kinds.
    pub fn source_entity_kinds<AnnotatedT>(&self, source_id: &SourceID) -> Result<&EntityKinds, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(self.dialect_entity_kinds(&self.get_source(source_id)?.dialect_id)?)
    }
}
