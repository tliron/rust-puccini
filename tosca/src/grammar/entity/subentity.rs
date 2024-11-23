use super::super::{depot::*, errors::*, name::*, source::*};

use compris::annotate::*;

//
// Subentity
//

/// Subentity.
///
/// This trait is only used for *contained* entities. Named entities should implement
/// [Entity](super::entity::Entity) instead.
pub trait Subentity<ParentT> {
    /// Complete.
    fn complete(
        &mut self,
        parent: &ParentT,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}
