use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::cli::depict::*,
};

//
// ImplementationDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An operation implementation definition specifies one or more artifacts (e.g. scripts) to be
/// used as the implementation for an operation in an interface.
///
/// A notification implementation definition specifies one or more artifacts to be used by the
/// orchestrator to subscribe and receive a particular notification (i.e. the artifact implements
/// the notification).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ImplementationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional implementation artifact (i.e., the primary script file within a TOSCA CSAR
    /// file).
    #[resolve(single)]
    #[depict(as(depict))]
    pub primary: Variant<AnnotatedT>,

    /// The optional list of one or more dependent or secondary implementation artifacts which are
    /// referenced by the primary implementation artifact (e.g., a library the script installs or
    /// a secondary script).
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub dependencies: Vec<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}
