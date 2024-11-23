use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    property_definition::*,
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
// CapabilityType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability type is a reusable entity that describes the properties and attributes of a
/// capability that a node type can declare to expose. Requirements that are declared as part of
/// one node can be fulfilled by the capabilities declared by another node.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityType<AnnotatedT>
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

    /// An optional map of property definitions for the capability type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the capability type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as
    /// valid sources of any relationship established to the declared capability type. If
    /// undefined, all node types are valid sources.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported
    /// as valid types of any relationship established to the declared capability type. If
    /// undefined, all relationship types are valid.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl_type_entity!(CapabilityType);

impl<AnnotatedT> Entity for CapabilityType<AnnotatedT>
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

        let parent = get_complete_parent!(CAPABILITY_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, parent, catalog, source_id, errors);

        if let Some((parent, scope)) = parent {
            errors_with_field_annotations!(
                errors, self, "valid_source_node_types",
                complete_types(
                    &mut self.valid_source_node_types,
                    &parent.valid_source_node_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );

            errors_with_field_annotations!(
                errors, self, "valid_relationship_types",
                complete_types(
                    &mut self.valid_relationship_types,
                    &parent.valid_relationship_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// CapabilityTypes
//

/// Map of [CapabilityType].
pub type CapabilityTypes<AnnotatedT> = BTreeMap<Name, CapabilityType<AnnotatedT>>;
