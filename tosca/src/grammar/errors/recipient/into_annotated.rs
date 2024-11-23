use super::super::tosca::*;

use {compris::annotate::*, kutil::std::error::*, std::marker::*};

//
// IntoAnnotatedErrorRecipient
//

/// An [ErrorRecipient] wrapper that calls [ToscaError::into_annotated] on all errors.
pub struct IntoAnnotatedErrorRecipient<'own, InnerAnnotatedT, InnerT>
where
    InnerT: ErrorRecipient<ToscaError<InnerAnnotatedT>>,
{
    /// Inner.
    pub inner: &'own mut InnerT,

    inner_annotated: PhantomData<InnerAnnotatedT>,
}

impl<'own, InnerAnnotatedT, InnerT, AnnotatedT> ErrorRecipient<ToscaError<AnnotatedT>>
    for IntoAnnotatedErrorRecipient<'own, InnerAnnotatedT, InnerT>
where
    InnerAnnotatedT: Annotated + Default,
    InnerT: ErrorRecipient<ToscaError<InnerAnnotatedT>>,
    AnnotatedT: Annotated + Default,
{
    fn give_error(&mut self, error: ToscaError<AnnotatedT>) -> Result<(), ToscaError<AnnotatedT>> {
        self.inner.give_error(error.into_annotated()).map_err(|error| error.into_annotated())
    }
}

//
// IntoAnnotated
//

/// Wrap in an [IntoAnnotatedErrorRecipient].
pub trait IntoAnnotated<'own, InnerAnnotatedT, InnerT>
where
    InnerT: ErrorRecipient<ToscaError<InnerAnnotatedT>>,
{
    /// Wrap in an [IntoAnnotatedErrorRecipient].
    fn into_annotated(self) -> IntoAnnotatedErrorRecipient<'own, InnerAnnotatedT, InnerT>;
}

impl<'own, InnerAnnotatedT, InnerT> IntoAnnotated<'own, InnerAnnotatedT, InnerT> for &'own mut InnerT
where
    InnerT: ErrorRecipient<ToscaError<InnerAnnotatedT>>,
{
    fn into_annotated(self) -> IntoAnnotatedErrorRecipient<'own, InnerAnnotatedT, InnerT> {
        IntoAnnotatedErrorRecipient { inner: self, inner_annotated: PhantomData }
    }
}
