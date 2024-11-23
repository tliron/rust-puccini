use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        interface_type::*,
        notification_definition::*,
        operation_definition::*,
        parameter_definition::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
    std::collections::*,
};

//
// InterfaceDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An interface definition defines an interface (containing operations and notifications
/// definitions) that can be associated with (i.e. defined within) a node or relationship type
/// definition. An interface definition may be refined in subsequent node or relationship type
/// derivations.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the interface type on which this interface definition is based.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for this interface definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// The optional map of input parameter refinements and new input parameter definitions
    /// available to all operations defined for this interface (the input parameters to be
    /// refined have been defined in the interface type definition).
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitionMap<AnnotatedT>,

    /// The optional map of operations refinements for this interface. The referred operations
    /// must have been defined in the interface type definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub operations: OperationDefinitions<AnnotatedT>,

    /// The optional map of notifications refinements for this interface. The referred operations
    /// must have been defined in the interface type definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub notifications: NotificationDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<InterfaceDefinition<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
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

        if let Some(interface_type) = depot
            .get_complete_entity::<InterfaceType<_>, _, _>(
                INTERFACE_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            let scope = &self.type_name.scope;

            errors_with_field_annotations!(
                errors, self, "inputs",
                complete_map(&mut self.inputs, &interface_type.inputs, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "operations",
                complete_map(&mut self.operations, &interface_type.operations, depot, source_id, scope, errors)?;
            );

            errors_with_field_annotations!(
                errors, self, "notifications",
                complete_map(&mut self.notifications, &interface_type.notifications, depot, source_id, scope, errors)?;
            );
        }

        errors_with_field_annotations!(
            errors, self, "inputs",
            complete_map(&mut self.inputs, &parent.inputs, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "operations",
            complete_map(&mut self.operations, &parent.operations, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "notifications",
            complete_map(&mut self.notifications, &parent.notifications, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<InterfaceDefinition<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            inputs: self.inputs.into_scoped(scope),
            operations: self.operations.into_scoped(scope),
            notifications: self.notifications.into_scoped(scope),
            annotations: self.annotations.clone(),
        }
    }
}

impl<'own, AnnotatedT> Definition<InterfaceType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, InterfaceType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.type_name.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            inputs: self.inputs.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.inputs),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            operations: self.operations.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.operations),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            notifications: self.notifications.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.type_.notifications),
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
            inputs: self.inputs.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.inputs),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            operations: self.operations.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.operations),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            notifications: self.notifications.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.notifications),
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

//
// InterfaceDefinitions
//

/// Map of [InterfaceDefinition].
pub type InterfaceDefinitions<AnnotatedT> = BTreeMap<ByteString, InterfaceDefinition<AnnotatedT>>;
