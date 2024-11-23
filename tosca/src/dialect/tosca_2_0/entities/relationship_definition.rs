use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        attribute_definition::*,
        interface_definition::*,
        property_definition::*,
        relationship_type::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
};

//
// RelationshipDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The relationship keyname in a requirement definition specifies a relationship definition that
/// provides information needed by TOSCA Orchestrators to construct a relationship to the TOSCA
/// node that contains the matching target capability.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory keyname used to provide the name of the relationship type used for the
    /// relationship.
    #[resolve(required, single, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description of the relationship definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	An optional map of property refinements for the relationship definition. The referred
    /// properties must have been defined in the relationship type definition referred by the type
    /// keyname. New properties may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the relationship definition. The referred
    /// attributes must have been defined in the relationship type definition referred by the
    /// type keyname. New attributes may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// The optional keyname used to define interface refinements for interfaces defined by the
    /// relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RelationshipDefinition<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
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

        if let Some(relationship_type) = depot
            .get_complete_entity::<RelationshipType<_>, _, _>(
                RELATIONSHIP_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            let scope = &self.type_name.scope;

            errors_with_field_annotations!(
                errors, self, "properties",
                complete_map(&mut self.properties, &relationship_type.properties, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "attributes",
                complete_map(&mut self.attributes, &relationship_type.attributes, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "interfaces",
                complete_map(&mut self.interfaces, &relationship_type.interfaces, depot, source_id, scope, errors)?;
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
            errors, self, "interfaces",
            complete_map(&mut self.interfaces, &parent.interfaces, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<RelationshipDefinition<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
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
            interfaces: self.interfaces.into_scoped(scope),
            annotations: self.annotations.clone(),
        }
    }
}

impl<'own, AnnotatedT> Definition<RelationshipType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, RelationshipType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
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
            interfaces: self.interfaces.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.interfaces),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
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
            interfaces: self.interfaces.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.interfaces),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.clone().in_scope(scope.clone());
    }
}
