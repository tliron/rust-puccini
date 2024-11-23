use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
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
// InterfaceType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An interface type is a reusable entity that describes a set of operations and notifications
/// that can be used to interact with or to manage a node or relationship in a TOSCA topology as
/// well as the input and output parameters used by those operations and notifications.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional map of input parameter definitions available to all operations defined for
    /// this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitions<AnnotatedT>,

    /// The optional map of operations defined for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub operations: OperationDefinitions<AnnotatedT>,

    /// The optional map of notifications defined for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub notifications: NotificationDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl_type_entity!(InterfaceType);

impl<AnnotatedT> Entity for InterfaceType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        catalog: &mut Catalog,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let parent = get_complete_parent!(INTERFACE_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map_for_field!(inputs, self, parent, catalog, source_id, errors);
        complete_map_for_field!(operations, self, parent, catalog, source_id, errors);
        complete_map_for_field!(notifications, self, parent, catalog, source_id, errors);

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// InterfaceTypes
//

/// Map of [InterfaceType].
pub type InterfaceTypes<AnnotatedT> = BTreeMap<Name, InterfaceType<AnnotatedT>>;
