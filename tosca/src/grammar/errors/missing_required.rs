use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MissingRequiredError
//

/// Missing required error.
#[derive(Debug, Error)]
pub struct MissingRequiredError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> MissingRequiredError<AnnotatedT>
where
    AnnotatedT: Default,
{
    /// Constructor.
    pub fn new(type_name: String, name: String) -> Self {
        Self { type_name, name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> MissingRequiredError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        MissingRequiredError { type_name: self.type_name, name: self.name, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(MissingRequiredError);

impl<AnnotatedT> Depict for MissingRequiredError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "missing required {}: {}", self.type_name, context.theme.error(name))
    }
}

impl<AnnotatedT> fmt::Display for MissingRequiredError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.type_name, self.name)
    }
}
