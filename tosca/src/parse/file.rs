use super::{super::grammar::*, errors::*};

use {
    compris::{annotate::*, normal::*, parse::*, *},
    std::io,
};

//
// FileVariant
//

/// File [Variant].
pub struct FileVariant<AnnotatedT> {
    /// Variant.
    pub variant: Variant<AnnotatedT>,

    /// Source.
    pub source_id: SourceID,

    /// Scope.
    pub scope: Scope,
}

impl<AnnotatedT> FileVariant<AnnotatedT> {
    /// Constructor.
    pub fn new(variant: Variant<AnnotatedT>, source_id: SourceID, scope: Scope) -> Self {
        Self { variant, source_id, scope }
    }

    /// Read.
    pub fn read<ReadT>(reader: &mut ReadT, source_id: SourceID, scope: Scope) -> Result<Self, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ReadT: io::Read,
    {
        let parser = Parser::new(Format::YAML).with_source(source_id.clone().into());
        Ok(Self::new(parser.parse(reader)?, source_id, scope))
    }
}
