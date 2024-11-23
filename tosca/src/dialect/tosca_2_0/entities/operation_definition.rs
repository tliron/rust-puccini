use super::{super::super::super::grammar::*, implementation_definition::*, parameter_definition::*};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
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
    pub inputs: ParameterDefinitions<AnnotatedT>,

    /// The optional map of parameter definitions for operation output values. Only as part of
    /// node and relationship type definitions, the output definitions may include mappings onto
    /// attributes of the node or relationship type that contains the definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub outputs: ParameterDefinitions<AnnotatedT>,

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
        _name: Option<ByteString>,
        parent: Option<(&Self, &Scope)>,
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

impl<AnnotatedT> ConvertIntoScope<OperationDefinition<AnnotatedT>> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            description: self.description.clone(),
            implementation: self.implementation.clone(),
            inputs: self.inputs.convert_into_scope(scope),
            outputs: self.outputs.convert_into_scope(scope),
            annotations: self.annotations.clone(),
        }
    }
}

//
// OperationDefinitions
//

/// Map of [OperationDefinition].
pub type OperationDefinitions<AnnotatedT> = BTreeMap<ByteString, OperationDefinition<AnnotatedT>>;
