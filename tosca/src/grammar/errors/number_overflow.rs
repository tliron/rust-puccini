use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// NumberOverflowError
//

/// Number overflow error.
#[derive(Debug, Error)]
pub struct NumberOverflowError<AnnotatedT> {
    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> NumberOverflowError<AnnotatedT> {
    /// Constructor.
    pub fn new() -> Self
    where
        AnnotatedT: Default,
    {
        Self { annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> NumberOverflowError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        NumberOverflowError::new().with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(NumberOverflowError);

impl<AnnotatedT> Depict for NumberOverflowError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, _context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "number overflow")
    }
}

impl<AnnotatedT> fmt::Display for NumberOverflowError<AnnotatedT> {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
