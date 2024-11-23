use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UnknownTypeError
//

/// Unknown type error.
#[derive(Debug, Error)]
pub struct UnknownTypeError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Context.
    pub context: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> UnknownTypeError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, context: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, context, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> UnknownTypeError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        UnknownTypeError { type_name: self.type_name, context: self.context, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(UnknownTypeError);

impl<AnnotatedT> Depict for UnknownTypeError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let type_name = format!("{:?}", self.type_name);
        write!(writer, "unknown type for {}: {}", self.context, context.theme.error(type_name))
    }
}

impl<AnnotatedT> fmt::Display for UnknownTypeError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}, {}", self.type_name, self.context)
    }
}
