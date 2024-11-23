use super::{
    super::{entity::*, errors::*, source::*},
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
    ///
    /// Note that we cannot allow the annotated type to be generic because this trait must be
    /// `dyn`-compatible.
    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;

    /// Initialize a [Source] [WithoutAnnotations].
    ///
    /// Note that we cannot allow the annotated type to be generic because this trait must be
    /// `dyn`-compatible.
    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithoutAnnotations>>;
}
