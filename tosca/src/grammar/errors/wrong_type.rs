use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// WrongTypeError
//

/// Wrong type error.
#[derive(Debug, Error)]
pub struct WrongTypeError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> WrongTypeError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> WrongTypeError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        WrongTypeError { type_name: self.type_name, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(WrongTypeError);

impl<AnnotatedT> Depict for WrongTypeError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let type_name = format!("{:?}", self.type_name);
        write!(writer, "wrong type: {}", context.theme.error(type_name))
    }
}

impl<AnnotatedT> fmt::Display for WrongTypeError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.type_name, formatter)
    }
}
