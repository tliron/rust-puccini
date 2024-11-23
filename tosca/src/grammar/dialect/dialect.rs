use super::{
    super::{depot::*, entity::*, errors::*, source::*},
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

    /// Supported [EntityKind]s.
    fn entity_kinds(&self) -> &EntityKinds;

    /// Initialize a [Source] [WithAnnotations].
    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;

    /// Initialize a [Source] [WithoutAnnotations].
    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithoutAnnotations>>;

    /// Compile a [Source] representing a TOSCA service template to a Floria
    /// [NodeTemplate](floria::NodeTemplate).
    ///
    /// Though only one [ID](floria::ID) is returned, the implementation may create many more
    /// Floria templates and groups.
    fn compile_source(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: floria::StoreRef,
        source_id: &SourceID,
        depot: &Depot,
        errors: ToscaErrorRecipientRef,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>>;
}
