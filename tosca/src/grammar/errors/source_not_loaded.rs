use super::super::source::*;

use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// SourceNotLoadedError
//

/// Source not loaded error.
#[derive(Debug, Error)]
pub struct SourceNotLoadedError<AnnotatedT> {
    /// Source ID.
    pub source_id: SourceID,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> SourceNotLoadedError<AnnotatedT> {
    /// Constructor.
    pub fn new(source_id: SourceID) -> Self
    where
        AnnotatedT: Default,
    {
        Self { source_id, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> SourceNotLoadedError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        SourceNotLoadedError { source_id: self.source_id, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(SourceNotLoadedError);

impl<AnnotatedT> Depict for SourceNotLoadedError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "source not loaded: {}", context.theme.error(&self.source_id))
    }
}

impl<AnnotatedT> fmt::Display for SourceNotLoadedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source_id, formatter)
    }
}
