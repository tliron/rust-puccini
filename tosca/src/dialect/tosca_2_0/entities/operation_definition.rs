use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        implementation_definition::*,
        interface_type::*,
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
// OperationDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An operation definition defines a function or procedure to which an operation implementation
/// can be bound.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description string for the associated operation.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional definition of the operation implementation. May not be used in an interface
    /// type definition (i.e. where an operation is initially defined), but only during refinements.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    /// The optional map of parameter definitions for operation input values.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitionMap<AnnotatedT>,

    /// The optional map of parameter definitions for operation output values. Only as part of
    /// node and relationship type definitions, the output definitions may include mappings onto
    /// attributes of the node or relationship type that contains the definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub outputs: ParameterDefinitionMap<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<OperationDefinition<AnnotatedT>> for OperationDefinition<AnnotatedT>
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

        if_none_clone(
            &mut self.implementation,
            &parent.implementation,
            &mut self.annotations,
            &parent.annotations,
            "implementation",
        );

        errors_with_field_annotations!(
            errors, self, "inputs",
            complete_map(&mut self.inputs, &parent.inputs, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "outputs",
            complete_map(&mut self.outputs, &parent.outputs, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<OperationDefinition<AnnotatedT>> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            description: self.description.clone(),
            implementation: self.implementation.clone(),
            inputs: self.inputs.into_scoped(scope),
            outputs: self.outputs.into_scoped(scope),
            annotations: self.annotations.clone(),
        }
    }
}

impl<'own, AnnotatedT> Definition<InterfaceType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        None
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, InterfaceType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        match context.type_.operations.get(context.definition_name) {
            Some(operation_definition) => self.derive(
                DefinitionDeriveContext {
                    definition_name: context.definition_name,
                    parent_definition: operation_definition,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            ),

            None => {
                errors.give(UndeclaredError::new("operation".into(), context.definition_name.into()))?;
                Ok(self.clone())
            }
        }
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
            description: complete_clone(&self.description, &context.parent_definition.description),
            implementation: complete_clone(&self.implementation, &context.parent_definition.implementation),
            inputs: self.inputs.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.inputs),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            outputs: self.outputs.complete(
                DefinitionsCompleteContext {
                    parent_definitions: Some(&context.parent_definition.outputs),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, _scope: &Scope) {}
}

//
// OperationDefinitions
//

/// Map of [OperationDefinition].
pub type OperationDefinitions<AnnotatedT> = BTreeMap<ByteString, OperationDefinition<AnnotatedT>>;
