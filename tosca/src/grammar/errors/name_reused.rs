use {
    compris::annotate::*,
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// NameReusedError
//

/// Name reused error.
#[derive(Debug, Error)]
pub struct NameReusedError<AnnotatedT> {
    /// Name.
    pub name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> NameReusedError<AnnotatedT> {
    /// Constructor.
    pub fn new(name: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { name, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> NameReusedError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        NameReusedError { name: self.name, annotated: Default::default() }.with_annotations_from(&self.annotated)
    }
}

impl_dyn_annotated_error!(NameReusedError);

impl<AnnotatedT> Depict for NameReusedError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let name = format!("{:?}", self.name);
        write!(writer, "name reused: {}", context.theme.error(name))
    }
}

impl<AnnotatedT> fmt::Display for NameReusedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name, formatter)
    }
}
