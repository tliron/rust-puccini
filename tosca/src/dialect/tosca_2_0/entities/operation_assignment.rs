use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        implementation_definition::*,
        operation_definition::*,
        parameter_definition::*,
        value_assignments::*,
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
// OperationAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An operation assignment may be used to assign values for input parameters, specify attribute
/// mappings for output parameters, and define/redefine the implementation definition of an already
/// defined operation in the interface definition. An operation assignment may be used inside
/// interface assignments inside node template or relationship template definitions (this includes
/// when operation assignments are part of a requirement assignment in a node template).
///
/// An operation assignment may add or change the implementation and description definition of the
/// operation. Assigning a value to an input parameter that had a fixed value specified during
/// operation definition or refinement is not allowed. Providing an attribute mapping for an output
/// parameter that was mapped during an operation refinement is also not allowed.
///
/// Note also that in the operation assignment we can use inputs and outputs that have not been
/// previously defined in the operation definition. This is equivalent to an ad-hoc definition of
/// a parameter, where the type is inferred from the assigned value (for input parameters) or from
/// the attribute to map to (for output parameters).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct OperationAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional definition of the operation implementation. Overrides implementation provided
    /// at operation definition.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    /// The optional map of parameter value assignments for assigning values to operation inputs.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub inputs: ValueAssignments<AnnotatedT>,

    /// The optional map of parameter mapping assignments that specify how operation outputs are
    /// mapped onto attributes of the node or relationship that contains the operation definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub outputs: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<OperationDefinition<AnnotatedT>> for OperationAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &OperationDefinition<AnnotatedT>,
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
            errors, self, "operations",
            complete_map(&mut self.outputs, &parent.outputs, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<OperationAssignment<AnnotatedT>> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> OperationAssignment<AnnotatedT> {
        OperationAssignment {
            implementation: self.implementation.clone(),
            inputs: self.inputs.into_scoped(scope),
            outputs: self.outputs.into_scoped(scope),
            annotations: clone_struct_annotations(&self.annotations, &["implementation", "inputs", "outputs"]),
            ..Default::default()
        }
    }
}

impl<'own, AnnotatedT> Assignment<OperationDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for OperationAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_assignment_entity_name() -> &'static str {
        "operation"
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, OperationDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            implementation: complete_clone(&self.implementation, &context.definition.implementation),
            inputs: self.inputs.complete_as_parameters(&context.definition.inputs, errors)?,
            outputs: self.outputs.complete_as_parameters(&context.definition.outputs, errors)?,
            annotations: Default::default(),
        })
    }

    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, OperationDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            implementation: context.definition.implementation.clone(),
            inputs: context.definition.inputs.to_assignments(errors)?,
            outputs: context.definition.outputs.to_assignments(errors)?,
            annotations: Default::default(),
        })
    }
}

//
// OperationAssignments
//

/// Map of [OperationAssignment].
pub type OperationAssignments<AnnotatedT> = BTreeMap<ByteString, OperationAssignment<AnnotatedT>>;
