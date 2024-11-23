use super::{
    super::super::super::grammar::*, artifact_type::*, capability_type::*, data_type::*, function_definition::*,
    group_type::*, import::*, interface_type::*, node_type::*, policy_type::*, relationship_type::*,
    repository_definition::*, service_template::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{cli::depict::*, std::immutable::*},
};

//
// File
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A TOSCA file can contain definitions of reusable building blocks for use in cloud applications,
/// complete models of cloud applications, or both. This section describes the top-level TOSCA
/// keynames—along with their grammars—that are allowed to appear in a TOSCA file.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct File<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Defines the version of the TOSCA specification used in this TOSCA file.
    #[resolve(required)]
    #[depict(as(display), style(name))]
    pub tosca_definitions_version: ByteString,

    /// Declares a description for this TOSCA file and its contents.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information. Domain-specific TOSCA profile
    /// specifications may define keynames that are mandatory for their implementations.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// Defines reusable YAML aliases (i.e., YAML alias anchors) for use throughout this TOSCA
    /// file.
    #[depict(option, as(depict))]
    pub dsl_definitions: Option<Variant<AnnotatedT>>,

    /// Declares a map of artifact type definitions for use in this TOSCA file and/or external
    /// TOSCA files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub artifact_types: ArtifactTypes<AnnotatedT>,

    /// Declares a map of TOSCA data type definitions for use in this TOSCA file and/or external
    /// TOSCA files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub data_types: DataTypes<AnnotatedT>,

    /// Declares a map of capability type definitions for use in this TOSCA file and/or external
    /// TOSCA files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub capability_types: CapabilityTypes<AnnotatedT>,

    /// Declares a map of interface type definitions for use in this TOSCA file and/or external
    /// TOSCA files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interface_types: InterfaceTypes<AnnotatedT>,

    /// Declares a map of relationship type definitions for use in this TOSCA file and/or external
    /// TOSCA files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub relationship_types: RelationshipTypes<AnnotatedT>,

    /// Declares a map of node type definitions for use in this TOSCA file and/or external TOSCA
    /// files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub node_types: NodeTypes<AnnotatedT>,

    /// Declares a map of group type definitions for use in this TOSCA file and/or external TOSCA
    /// files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub group_types: GroupTypes<AnnotatedT>,

    /// Declares a map of policy type definitions for use in this TOSCA file and/or external TOSCA
    /// files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub policy_types: PolicyTypes<AnnotatedT>,

    /// Declares a map of external repositories that contain artifacts that are referenced in this
    /// TOSCA file along with the addresses used to connect to them in order to retrieve the
    /// artifacts.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub repositories: RepositoryDefinitions<AnnotatedT>,

    /// Declares a map of function definitions for use in this TOSCA file and/or external TOSCA
    /// files.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub functions: FunctionDefinitions<AnnotatedT>,

    /// The profile name that can be used by other TOSCA files to import the type definitions in
    /// this document.
    #[resolve]
    #[depict(option, style(string))]
    pub profile: Option<ByteString>,

    /// Declares a list of import statements pointing to external TOSCA files or well-known profiles.
    /// For example, these may be file locations or URIs relative to the TOSCA file within the same
    /// TOSCA CSAR file.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub imports: Imports<AnnotatedT>,

    /// Defines a template from which to create a mode/representation of an application or
    /// service. Service templates consist of node templates that represent the application's or
    /// service's components, as well as relationship templates representing relations between
    /// these components.
    #[resolve]
    #[depict(option, as(depict))]
    pub service_template: Option<ServiceTemplate<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}
