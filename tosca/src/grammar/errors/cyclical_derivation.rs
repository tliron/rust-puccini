use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// CyclicalDerivationError
//

/// Cyclical derivation error.
#[derive(Debug, Error)]
pub struct CyclicalDerivationError<AnnotatedT> {
    /// Parent name.
    pub parent_name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> CyclicalDerivationError<AnnotatedT> {
    /// Constructor.
    pub fn new(parent_name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { parent_name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> CyclicalDerivationError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        CyclicalDerivationError { parent_name: self.parent_name, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(CyclicalDerivationError);

impl<AnnotatedT> Depict for CyclicalDerivationError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let parent_name = format!("{:?}", self.parent_name);
        write!(writer, "cyclical derivation: {}", context.theme.error(parent_name))
    }
}

impl<AnnotatedT> fmt::Display for CyclicalDerivationError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.parent_name, formatter)
    }
}
