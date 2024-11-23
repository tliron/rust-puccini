use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        attribute_definition::*,
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
// CapabilityType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability type is a reusable entity that describes the properties and attributes of a
/// capability that a node type can declare to expose. Requirements that are declared as part of
/// one node can be fulfilled by the capabilities declared by another node.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityType<AnnotatedT>
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

    /// An optional map of property definitions for the capability type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the capability type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as
    /// valid sources of any relationship established to the declared capability type. If
    /// undefined, all node types are valid sources.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported
    /// as valid types of any relationship established to the declared capability type. If
    /// undefined, all relationship types are valid.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for CapabilityType<AnnotatedT>
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
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        if let Some(derived_from) = &self.derived_from {
            let Some(parent) = depot
                .get_complete_entity_next::<Self, _, _>(
                    CAPABILITY_TYPE,
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

            errors_with_field_annotations!(
                errors, self, "valid_relationship_types",
                complete_types(
                    &mut self.valid_relationship_types,
                    &parent.valid_relationship_types,
                    depot,
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

impl<'own, AnnotatedT> Type<OldCatalog<'own, AnnotatedT>, AnnotatedT> for CapabilityType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_entity_name() -> &'static str {
        "CapabilityType"
    }

    fn get_floria_group_id_prefix() -> &'static str {
        "capability"
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
            valid_source_node_types: complete_type_names_from(
                &self.valid_source_node_types,
                context.parent_type,
                |entity| &entity.valid_source_node_types,
            ),
            valid_relationship_types: complete_type_names_from(
                &self.valid_relationship_types,
                context.parent_type,
                |entity| &entity.valid_relationship_types,
            ),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// CapabilityTypes
//

/// Map of [CapabilityType].
pub type CapabilityTypes<AnnotatedT> = BTreeMap<Name, CapabilityType<AnnotatedT>>;
