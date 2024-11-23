use super::{
    super::{catalog::*, entity::*, errors::*, source::*},
    id::*,
};

use {
    compris::{annotate::*, normal::*},
    std::any::*,
};

//
// Dialect
//

/// Dialect.
pub trait Dialect
where
    Self: Any,
{
    /// Dialect ID.
    fn dialect_id(&self) -> DialectID;

    /// Supported entity kinds.
    fn entity_kinds(&self) -> &EntityKinds;

    /// Initialize a source with annotations.
    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;

    /// Initialize a source without annotations.
    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithoutAnnotations>>;

    /// Compile a source representing a TOSCA service template to a Floria
    /// [VertexTemplate](floria::VertexTemplate).
    ///
    /// Though only one Floria ID is returned, the implementation may create other Floria entities.
    fn compile_source(
        &self,
        directory: &floria::Directory,
        store: floria::StoreRef,
        source_id: &SourceID,
        catalog: &Catalog,
        errors: ToscaErrorRecipientRef,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>>;
}
