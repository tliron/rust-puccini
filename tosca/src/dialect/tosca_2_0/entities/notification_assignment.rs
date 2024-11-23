use super::{
    super::super::super::grammar::*, implementation_definition::*, notification_definition::*, value_assignment::*,
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
// NotificationAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A notification assignment may be used to specify attribute mappings for output parameters and
/// to define/redefine the implementation definition and description definition of an already
/// defined notification in the interface definition. A notification assignment may be used inside
/// interface assignments which are themselves inside node or relationship template definitions
/// (this includes when notification assignments are part of a requirement assignment in a node
/// template).
///
/// Providing an attribute mapping for an output parameter that was mapped during a previous
/// refinement is not allowed. Note also that in the notification assignment we can use outputs
/// that have not been previously defined in the operation definition. This is equivalent to an
/// ad-hoc definition of an output parameter, where the type is inferred from the attribute to map
/// to.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NotificationAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional definition of the notification implementation. Overrides implementation provided at notification definition.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    /// The optional map of parameter value assignments for assigning values to notification inputs.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub inputs: ValueAssignments<AnnotatedT>,

    /// The optional map of parameter mapping assignments that specify how notification outputs values are mapped onto attributes of the node or relationship type that contains the notification definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub outputs: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<NotificationDefinition<AnnotatedT>> for NotificationAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<(&NotificationDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_map_for_field!(inputs, self, parent, catalog, source_id, errors);
        complete_map_for_field!(outputs, self, parent, catalog, source_id, errors);

        if let Some((parent, _scope)) = parent {
            if_none_clone!(implementation, self, parent);
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<NotificationAssignment<AnnotatedT>> for NotificationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> NotificationAssignment<AnnotatedT> {
        NotificationAssignment {
            implementation: self.implementation.clone(),
            inputs: self.inputs.convert_into_scope(scope),
            outputs: self.outputs.convert_into_scope(scope),
            annotations: clone_struct_annotations(&self.annotations, &["implementation", "inputs", "outputs"]),
            ..Default::default()
        }
    }
}

//
// NotificationAssignments
//

/// Map of [NotificationAssignment].
pub type NotificationAssignments<AnnotatedT> = BTreeMap<ByteString, NotificationAssignment<AnnotatedT>>;
