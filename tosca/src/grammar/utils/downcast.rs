use super::super::errors::*;

use {kutil::std::any::*, std::any::*};

//
// DowncastRefOrError
//

/// Downcast to an [Any] reference or return an error.
pub trait DowncastRefOrError {
    /// Downcast to an [Any] reference or return an error.
    fn downcast_ref_or_error<AnyT, AnnotatedT>(&self, type_name: String) -> Result<&AnyT, WrongTypeError<AnnotatedT>>
    where
        AnyT: Any,
        AnnotatedT: Default;
}

impl<DowncastRefT> DowncastRefOrError for DowncastRefT
where
    DowncastRefT: DowncastRef,
{
    fn downcast_ref_or_error<AnyT, AnnotatedT>(&self, type_name: String) -> Result<&AnyT, WrongTypeError<AnnotatedT>>
    where
        AnyT: Any,
        AnnotatedT: Default,
    {
        self.downcast_ref().ok_or_else(|| WrongTypeError::new(type_name))
    }
}
