use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    artifact_definition::*,
    attribute_definition::*,
    capability_definition::*,
    interface_definition::*,
    property_definition::*,
    requirement_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// NodeType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A node type is a reusable entity that defines the structure of observable properties and
/// attributes of a node, the capabilities and requirements of that node, as well as its
/// supported interfaces and the artifacts it uses.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NodeType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    ///	An optional map of property definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional map of capability definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub capabilities: CapabilityDefinitions<AnnotatedT>,

    /// An optional list of requirement definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub requirements: RequirementDefinitions<AnnotatedT>,

    /// An optional map of interface definitions supported by the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    /// An optional map of artifact definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub artifacts: ArtifactDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl_type_entity!(NodeType);

impl<AnnotatedT> Entity for NodeType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        catalog: &mut Catalog,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let parent = get_complete_parent!(NODE_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, parent, catalog, source_id, errors);
        complete_map_for_field!(capabilities, self, parent, catalog, source_id, errors);
        complete_tagged_values_for_field!(requirements, self, parent, catalog, source_id, errors);
        complete_map_for_field!(interfaces, self, parent, catalog, source_id, errors);
        complete_map_for_field!(artifacts, self, parent, catalog, source_id, errors);

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// NodeTypes
//

/// Map of [NodeType].
pub type NodeTypes<AnnotatedT> = BTreeMap<Name, NodeType<AnnotatedT>>;
