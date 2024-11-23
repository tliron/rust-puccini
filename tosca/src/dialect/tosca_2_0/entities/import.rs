use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::debug::*, std::zerocopy::*},
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
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct Import<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The url that references a TOSCA file to be imported. An import statement must
    /// include either a URL or a profile, but not both.
    #[resolve(single)]
    #[debuggable(option, style(string), tag = tag::span)]
    pub url: Option<ByteString>,

    /// The profile name that references a named type profile to be imported. An import
    /// statement must include either a URL or a profile, but not both.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub profile: Option<ByteString>,

    /// The optional symbolic name of the repository definition where the imported file
    /// can be found as a string. The repository name can only be used when a URL is
    /// specified.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub repository: Option<Name>,

    /// The optional name of the namespace into which to import the type definitions
    /// from the imported template or profile.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub namespace: Option<Name>,

    /// Declares a description for the import definition.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information about the import
    /// definition.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

//
// Imports
//

/// Vector of [Import].
pub type Imports<AnnotatedT> = Vec<Import<AnnotatedT>>;
