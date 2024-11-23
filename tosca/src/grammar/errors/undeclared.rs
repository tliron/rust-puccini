use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UndeclaredError
//

/// Undeclared error.
#[derive(Debug, Error)]
pub struct UndeclaredError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> UndeclaredError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> UndeclaredError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        UndeclaredError { type_name: self.type_name, name: self.name, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(UndeclaredError);

impl<AnnotatedT> Depict for UndeclaredError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "undeclared {}: {}", context.theme.name(&self.type_name), context.theme.error(name))
    }
}

impl<AnnotatedT> fmt::Display for UndeclaredError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.type_name, self.name)
    }
}
