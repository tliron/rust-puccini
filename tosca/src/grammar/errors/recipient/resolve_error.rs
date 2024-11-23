use super::super::tosca::*;

use {compris::resolve::*, kutil::std::error::*, std::marker::*};

//
// ResolveErrorRecipient
//

/// An [ErrorRecipient] wrapper that converts [ResolveError] to [ToscaError].
pub struct ResolveErrorRecipient<AnnotatedT, InnerT> {
    /// Inner.
    pub inner: InnerT,

    annotated: PhantomData<AnnotatedT>,
}

impl<AnnotatedT, InnerT> ErrorRecipient<ResolveError<AnnotatedT>> for ResolveErrorRecipient<AnnotatedT, InnerT>
where
    InnerT: ErrorRecipient<ToscaError<AnnotatedT>>,
{
    fn give_error(&mut self, error: ResolveError<AnnotatedT>) -> Result<(), ResolveError<AnnotatedT>> {
        // We make the reasonable assumption that if the inner returns a ResolveError then it's *our* ResolveError
        self.inner.give_error(error.into()).or_else(|error| match error {
            ToscaError::Resolve(error) => Err(error),
            _ => Ok(()),
        })
    }
}

//
// ToResolveErrorRecipient
//

/// Wrap in a [ResolveErrorRecipient].
pub trait ToResolveErrorRecipient<AnnotatedT, ErrorRecipientT> {
    /// Wrap in a [ResolveErrorRecipient].
    fn to_resolve_error_recipient(self) -> ResolveErrorRecipient<AnnotatedT, ErrorRecipientT>;
}

impl<AnnotatedT, ErrorRecipientT> ToResolveErrorRecipient<AnnotatedT, ErrorRecipientT> for ErrorRecipientT
where
    ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
{
    fn to_resolve_error_recipient(self) -> ResolveErrorRecipient<AnnotatedT, ErrorRecipientT> {
        ResolveErrorRecipient { inner: self, annotated: PhantomData }
    }
}
