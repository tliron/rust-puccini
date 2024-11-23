use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        notification_definition::*,
        operation_definition::*,
        parameter_definition::*,
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
// InterfaceType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An interface type is a reusable entity that describes a set of operations and notifications
/// that can be used to interact with or to manage a node or relationship in a TOSCA topology as
/// well as the input and output parameters used by those operations and notifications.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// The optional map of input parameter definitions available to all operations defined for
    /// this interface.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub inputs: ParameterDefinitionMap<AnnotatedT>,

    /// The optional map of operations defined for this interface.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub operations: OperationDefinitions<AnnotatedT>,

    /// The optional map of notifications defined for this interface.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub notifications: NotificationDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for InterfaceType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        depot: &mut Depot,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);

        let Some(derived_from) = &self.derived_from else {
            self.completion = Completion::Complete;
            return Ok(());
        };

        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let Some(parent) = depot
            .get_complete_entity_next::<Self, _, _>(
                INTERFACE_TYPE,
                derived_from,
                source_id,
                callstack,
                &mut errors.with_field_annotations(self, "derived_from"),
            )?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &derived_from.scope;

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

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Type<OldCatalog<'own, AnnotatedT>, AnnotatedT> for InterfaceType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_entity_name() -> &'static str {
        "InterfaceType"
    }

    fn get_floria_group_id_prefix() -> &'static str {
        "interface"
    }

    fn get_version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    fn get_description(&self) -> Option<&ByteString> {
        self.description.as_ref()
    }

    fn get_metadata(&self) -> &Metadata<AnnotatedT> {
        &self.metadata
    }

    fn get_parent_name(&self) -> Option<&FullName> {
        self.derived_from.as_ref()
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: TypeCompleteContext<'_, Self, OldCatalog<'_, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            derived_from: self.derived_from.clone(),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            inputs: self.inputs.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.inputs),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            operations: self.operations.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.operations),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            notifications: self.notifications.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.notifications),
                    types: &context.catalog.interface_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// InterfaceTypes
//

/// Map of [InterfaceType].
pub type InterfaceTypes<AnnotatedT> = BTreeMap<Name, InterfaceType<AnnotatedT>>;
