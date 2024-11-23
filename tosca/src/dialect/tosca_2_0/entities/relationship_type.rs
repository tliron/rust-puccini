use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        attribute_definition::*,
        interface_definition::*,
        property_definition::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::debug::*,
        std::{error::*, zerocopy::*},
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
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    ///	An optional map of property definitions for the relationship type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the relationship type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional map of interface definitions supported by the relationship type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    /// An optional list of one or more names of capability types that are valid targets
    /// for this relationship. If undefined, all capability types are valid.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_capability_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid targets for
    /// this relationship. If undefined, all node types are valid targets.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_target_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid sources for
    /// this relationship. If undefined, all node types are valid sources.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_source_node_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for RelationshipType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        depot: &mut Depot,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);

        let Some(derived_from) = &self.derived_from else {
            self.completion = Completion::Complete;
            return Ok(());
        };

        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let Some(parent) = depot
            .get_complete_entity_next::<Self, _, _>(
                RELATIONSHIP_TYPE,
                derived_from,
                source_id,
                callstack,
                &mut errors.with_field_annotations(self, "derived_from"),
            )?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &derived_from.scope;

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "attributes",
            complete_map(&mut self.attributes, &parent.attributes, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "valid_capability_types",
            complete_types(
                &mut self.valid_capability_types,
                &parent.valid_capability_types,
                depot,
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
                depot,
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
                depot,
                source_id,
                scope,
                errors,
            )?;
        );

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Type<OldCatalog<'own, AnnotatedT>, AnnotatedT> for RelationshipType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_entity_name() -> &'static str {
        "RelationshipType"
    }

    fn get_floria_group_id_prefix() -> &'static str {
        "relationship"
    }

    fn get_version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    fn get_description(&self) -> Option<&ByteString> {
        self.description.as_ref()
    }

    fn get_metadata(&self) -> &Metadata<AnnotatedT> {
        &self.metadata
    }

    fn get_parent_name(&self) -> Option<&FullName> {
        self.derived_from.as_ref()
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: TypeCompleteContext<'_, Self, OldCatalog<'_, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            derived_from: self.derived_from.clone(),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            properties: self.properties.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.properties),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            attributes: self.attributes.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.attributes),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            interfaces: self.interfaces.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.interfaces),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            valid_capability_types: complete_type_names_from(
                &self.valid_capability_types,
                context.parent_type,
                |entity| &entity.valid_capability_types,
            ),
            valid_target_node_types: complete_type_names_from(
                &self.valid_target_node_types,
                context.parent_type,
                |entity| &entity.valid_target_node_types,
            ),
            valid_source_node_types: complete_type_names_from(
                &self.valid_source_node_types,
                context.parent_type,
                |entity| &entity.valid_source_node_types,
            ),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// RelationshipTypes
//

/// Map of [RelationshipType].
pub type RelationshipTypes<AnnotatedT> = BTreeMap<Name, RelationshipType<AnnotatedT>>;
