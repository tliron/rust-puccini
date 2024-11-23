use super::{
    super::{super::super::grammar::*, dialect::*},
    interface_type::*,
    notification_definition::*,
    operation_definition::*,
    parameter_definition::*,
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
    pub inputs: ParameterDefinitions<AnnotatedT>,

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
        _name: Option<ByteString>,
        parent: Option<(&Self, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        if let Some((parent, _scope)) = &parent {
            if self.type_name.is_empty() && !parent.type_name.is_empty() {
                self.type_name = parent.type_name.clone();
            } else {
                validate_type_name(&self.type_name, &parent.type_name, catalog, errors)?;
            }
        }

        let interface_type =
            get_complete_entity!(INTERFACE_TYPE, InterfaceType, self, type_name, catalog, source_id, errors);

        complete_map_for_field!(inputs, self, interface_type, catalog, source_id, errors);
        complete_map_for_field!(inputs, self, parent, catalog, source_id, errors);
        complete_map_for_field!(operations, self, interface_type, catalog, source_id, errors);
        complete_map_for_field!(operations, self, parent, catalog, source_id, errors);
        complete_map_for_field!(notifications, self, interface_type, catalog, source_id, errors);
        complete_map_for_field!(notifications, self, parent, catalog, source_id, errors);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<InterfaceDefinition<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            inputs: self.inputs.convert_into_scope(scope),
            operations: self.operations.convert_into_scope(scope),
            notifications: self.notifications.convert_into_scope(scope),
            annotations: self.annotations.clone(),
        }
    }
}

//
// InterfaceDefinitions
//

/// Map of [InterfaceDefinition].
pub type InterfaceDefinitions<AnnotatedT> = BTreeMap<ByteString, InterfaceDefinition<AnnotatedT>>;
