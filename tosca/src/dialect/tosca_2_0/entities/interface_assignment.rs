use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        interface_definition::*,
        notification_assignment::*,
        operation_assignment::*,
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
// InterfaceAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An interface assignment is used to specify assignments for the inputs, operations and
/// notifications defined in the interface. Interface assignments may be used within a node or
/// relationship template definition (including when interface assignments are referenced as part
/// of a requirement assignment in a node template).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional map of input parameter assignments. Template authors MAY provide parameter
    /// assignments for interface inputs that are not defined in their corresponding interface
    /// type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub inputs: ValueAssignments<AnnotatedT>,

    /// The optional map of operations assignments specified for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub operations: OperationAssignments<AnnotatedT>,

    /// The optional map of notifications assignments specified for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub notifications: NotificationAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<InterfaceDefinition<AnnotatedT>> for InterfaceAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &InterfaceDefinition<AnnotatedT>,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

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

impl<AnnotatedT> IntoScoped<InterfaceAssignment<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> InterfaceAssignment<AnnotatedT> {
        InterfaceAssignment {
            inputs: self.inputs.into_scoped(scope),
            operations: self.operations.into_scoped(scope),
            notifications: self.notifications.into_scoped(scope),
            annotations: clone_struct_annotations(&self.annotations, &["inputs", "operations", "notifications"]),
            ..Default::default()
        }
    }
}

impl<'own, AnnotatedT> Assignment<InterfaceDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for InterfaceAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_assignment_entity_name() -> &'static str {
        "interface"
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, InterfaceDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            inputs: context.definition.inputs.to_assignments(errors)?,
            operations: self.operations.complete(
                AssignmentsAsMapCompleteContext {
                    definitions: &context.definition.operations,
                    catalog: context.catalog,
                },
                errors,
            )?,
            notifications: self.notifications.complete(
                AssignmentsAsMapCompleteContext {
                    definitions: &context.definition.notifications,
                    catalog: context.catalog,
                },
                errors,
            )?,
            annotations: Default::default(),
        })
    }

    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, InterfaceDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            inputs: context.definition.inputs.to_assignments(errors)?,
            operations: AssignmentsAsMap::from_definitions(
                AssignmentsAsMapFromDefinitionsContext {
                    definitions: &context.definition.operations,
                    catalog: context.catalog,
                },
                errors,
            )?,
            notifications: AssignmentsAsMap::from_definitions(
                AssignmentsAsMapFromDefinitionsContext {
                    definitions: &context.definition.notifications,
                    catalog: context.catalog,
                },
                errors,
            )?,
            annotations: Default::default(),
        })
    }
}

//
// InterfaceAssignments
//

/// Map of [InterfaceAssignment].
pub type InterfaceAssignments<AnnotatedT> = BTreeMap<ByteString, InterfaceAssignment<AnnotatedT>>;
