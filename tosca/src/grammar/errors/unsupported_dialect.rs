use super::super::dialect::*;

use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UnsupportedDialectError
//

/// Unsupported dialect error.
#[derive(Debug, Error)]
pub struct UnsupportedDialectError<AnnotatedT> {
    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> UnsupportedDialectError<AnnotatedT> {
    /// Constructor.
    pub fn new(dialect_id: DialectID) -> Self
    where
        AnnotatedT: Default,
    {
        Self { dialect_id, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> UnsupportedDialectError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        UnsupportedDialectError { dialect_id: self.dialect_id, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(UnsupportedDialectError);

impl<AnnotatedT> Depict for UnsupportedDialectError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "unsupported dialect: {}", context.theme.error(&self.dialect_id))
    }
}

impl<AnnotatedT> fmt::Display for UnsupportedDialectError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.dialect_id, formatter)
    }
}
