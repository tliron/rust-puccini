use super::super::errors::*;

use {
    compris::{annotate::*, resolve::*},
    kutil::std::error::*,
};

//
// ResolveWithToscaErrors
//

/// Helper to call [Resolve::resolve_with_errors] with a [ToscaErrorRecipientRef].
pub trait ResolveWithToscaErrors<ResolvedT, AnnotatedT> {
    /// Helper to call [Resolve::resolve_with_errors] with a [ToscaErrorRecipientRef].
    fn resolve_with_tosca_errors(
        &self,
        errors: ToscaErrorRecipientRef,
    ) -> Result<Option<ResolvedT>, ToscaError<AnnotatedT>>;
}

impl<ResolveT, ResolvedT, AnnotatedT> ResolveWithToscaErrors<ResolvedT, AnnotatedT> for ResolveT
where
    ResolveT: Resolve<ResolvedT, AnnotatedT>,
    AnnotatedT: Annotated + Default + Clone,
{
    fn resolve_with_tosca_errors(
        &self,
        errors: ToscaErrorRecipientRef,
    ) -> Result<Option<ResolvedT>, ToscaError<AnnotatedT>> {
        Ok(self.resolve_with_errors(&mut errors.to_error_recipient().into_annotated().to_resolve_error_recipient())?)
    }
}
