use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    interface_definition::*,
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
// RelationshipType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A relationship type is a reusable entity that defines the structure of observable properties
/// and attributes of a relationship as well as its supported interfaces.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipType<AnnotatedT>
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

    ///	An optional map of property definitions for the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional map of interface definitions supported by the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    /// An optional list of one or more names of capability types that are valid targets
    /// for this relationship. If undefined, all capability types are valid.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_capability_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid targets for
    /// this relationship. If undefined, all node types are valid targets.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_target_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid sources for
    /// this relationship. If undefined, all node types are valid sources.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl_type_entity!(RelationshipType);

impl<AnnotatedT> Entity for RelationshipType<AnnotatedT>
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

        let parent = get_complete_parent!(RELATIONSHIP_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, parent, catalog, source_id, errors);
        complete_map_for_field!(interfaces, self, parent, catalog, source_id, errors);

        if let Some((parent, scope)) = parent {
            errors_with_field_annotations!(
                errors, self, "valid_capability_types",
                complete_types(
                    &mut self.valid_capability_types,
                    &parent.valid_capability_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );

            errors_with_field_annotations!(
                errors, self, "valid_target_node_types",
                complete_types(
                    &mut self.valid_target_node_types,
                    &parent.valid_target_node_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );

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
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// RelationshipTypes
//

/// Map of [RelationshipType].
pub type RelationshipTypes<AnnotatedT> = BTreeMap<Name, RelationshipType<AnnotatedT>>;
