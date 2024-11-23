use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::depict::*, std::immutable::*},
};

//
// Import
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// Import definitions are used within a TOSCA file to uniquely identify and locate other TOSCA
/// files that have type, repository, and function definitions to be imported (included) into
/// this TOSCA file.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct Import<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The url that references a TOSCA file to be imported. An import statement must
    /// include either a URL or a profile, but not both.
    #[resolve(single)]
    #[depict(option, style(string))]
    pub url: Option<ByteString>,

    /// The profile name that references a named type profile to be imported. An import
    /// statement must include either a URL or a profile, but not both.
    #[resolve]
    #[depict(option, style(string))]
    pub profile: Option<ByteString>,

    /// The optional symbolic name of the repository definition where the imported file
    /// can be found as a string. The repository name can only be used when a URL is
    /// specified.
    #[resolve]
    #[depict(option, style(string))]
    pub repository: Option<Name>,

    /// The optional name of the namespace into which to import the type definitions
    /// from the imported template or profile.
    #[resolve]
    #[depict(option, style(string))]
    pub namespace: Option<Name>,

    /// Declares a description for the import definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information about the import
    /// definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

//
// Imports
//

/// Vector of [Import].
pub type Imports<AnnotatedT> = Vec<Import<AnnotatedT>>;
