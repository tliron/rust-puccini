use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// RepositoryDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A repository definition defines an external repository that contains TOSCA files and/or
/// artifacts that are referenced or imported by this TOSCA file.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
pub struct RepositoryDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Declares a description for the repository being defined.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// The URL or network address used to access the repository.
    #[resolve]
    #[depict(style(string))]
    pub url: ByteString,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for RepositoryDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        _catalog: &mut Catalog,
        _source_id: &SourceID,
        _callstack: &mut CallStack,
        _errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// RepositoryDefinitions
//

/// Map of [RepositoryDefinition].
pub type RepositoryDefinitions<AnnotatedT> = BTreeMap<Name, RepositoryDefinition<AnnotatedT>>;
