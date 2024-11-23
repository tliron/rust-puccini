use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        attribute_definition::*,
        capability_type::*,
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
// CapabilityDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability definition defines a typed set of data that a node can expose and that is used to
/// describe a relevant feature of the component described by the node that can be used to fulfill
/// a requirement exposed by another node. A capability is defined as part of a node type
/// definition and may be refined during node type derivation.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the capability type on which this capability definition is based.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// The optional description of the Capability definition.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property refinements for the capability definition. The referred
    /// properties must have been defined in the capability type definition referred to by the
    /// type keyname. New properties may not be added.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the capability definition. The referred
    /// attributes must have been defined in the capability type definition referred by the type
    /// keyname. New attributes may not be added.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as valid
    /// sources of any relationship established to the declared capability type. If undefined, all
    /// node types are valid sources. If valid_source_node_types is defined in the capability type,
    /// each element in this list must either be or derived from an element in the list defined in
    /// the type.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported as
    /// valid types of any relationship established to the declared capability type. If undefined,
    /// all relationship types are valid. If valid_relationship_types is defined in the capability
    /// type, each element in this list must either be or derived from an element in the list
    /// defined in the type.
    #[resolve]
    #[debuggable(option, iter(item), as(debuggable), tag = tag::span)]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<CapabilityDefinition<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &Self,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        if let Some(capability_type) = depot
            .get_complete_entity::<CapabilityType<_>, _, _>(
                CAPABILITY_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            let scope = &self.type_name.scope;

            errors_with_field_annotations!(
                errors, self, "properties",
                complete_map(&mut self.properties, &capability_type.properties, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "attributes",
                complete_map(&mut self.attributes, &capability_type.attributes, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "valid_source_node_types",
                complete_types(
                    &mut self.valid_source_node_types,
                    &capability_type.valid_source_node_types,
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
                    &capability_type.valid_relationship_types,
                    depot,
                    source_id,
                    scope,
                    errors,
                )?;
            );
        }

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

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<CapabilityDefinition<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            properties: self.properties.into_scoped(scope),
            attributes: self.attributes.into_scoped(scope),
            valid_source_node_types: self.valid_source_node_types.into_scoped(scope),
            valid_relationship_types: self.valid_relationship_types.into_scoped(scope),
            annotations: self.annotations.clone(),
        }
    }
}

impl<'own, AnnotatedT> Definition<CapabilityType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, CapabilityType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.type_name.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            properties: self.properties.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.properties),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            attributes: self.attributes.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.attributes),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            valid_source_node_types: complete_type_names(
                &self.valid_source_node_types,
                &context.type_.valid_source_node_types,
            ),
            valid_relationship_types: complete_type_names(
                &self.valid_relationship_types,
                &context.type_.valid_relationship_types,
            ),
            annotations: self.annotations.clone(),
        })
    }

    fn derive<ErrorRecipientT>(
        &self,
        context: DefinitionDeriveContext<'_, Self, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: self.type_name.clone(),
            description: complete_clone(&self.description, &context.parent_definition.description),
            metadata: self.metadata.clone(),
            properties: self.properties.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.properties),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            attributes: self.attributes.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.attributes),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            valid_source_node_types: complete_type_names(
                &self.valid_source_node_types,
                &context.parent_definition.valid_source_node_types,
            ),
            valid_relationship_types: complete_type_names(
                &self.valid_relationship_types,
                &context.parent_definition.valid_relationship_types,
            ),
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.clone().in_scope(scope.clone());
    }
}

//
// CapabilityDefinitions
//

/// Map of [CapabilityDefinition].
pub type CapabilityDefinitions<AnnotatedT> = BTreeMap<ByteString, CapabilityDefinition<AnnotatedT>>;
