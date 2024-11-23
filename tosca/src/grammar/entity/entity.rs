use super::{
    super::{depot::*, errors::*, source::*},
    call_stack::*,
    completion::*,
};

use {compris::annotate::*, kutil::cli::debug::*, std::any::*};

//
// Entity
//

/// Entity.
///
/// This trait is only used for *named* entities. Contained entities should implement
/// [Subentity](super::subentity::Subentity) instead.
pub trait Entity
where
    Self: Any + DynDebuggable,
{
    /// The completion status.
    fn completion(&self) -> Completion;

    /// Whether the entity is complete.
    fn is_complete(&self) -> bool {
        self.completion() == Completion::Complete
    }

    /// Whether the entity should be completed.
    fn should_complete(&self) -> bool {
        self.completion() == Completion::Incomplete
    }

    /// Complete.
    ///
    /// Note that we cannot allow the annotated type to be generic because this trait must be
    /// `dyn`-compatible.
    ///
    /// If you need a different annotated type for the errors you can use
    /// [IntoAnnotated::into_annotated].
    fn complete(
        &mut self,
        depot: &mut Depot,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}
