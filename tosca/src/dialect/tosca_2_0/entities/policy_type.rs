use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        property_definition::*,
        trigger_definition::*,
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
// PolicyType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A policy type defines a type of a policy that affects or governs an application or service's
/// topology at some stage of its lifecycle but is not explicitly part of the topology itself
/// (i.e., it does not prevent the application or service from being deployed or run if it did
/// not exist).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PolicyType<AnnotatedT>
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

    ///	An optional map of property definitions for the policy type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional list of valid node types or group types the policy type can be applied to.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub targets: Option<Vec<FullName>>,

    /// An optional map of policy triggers for the policy type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub triggers: TriggerDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for PolicyType<AnnotatedT>
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
                POLICY_TYPE,
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
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "targets",
            complete_types(&mut self.targets, &parent.targets, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "triggers",
            complete_map(&mut self.triggers, &parent.triggers, depot, source_id, scope, errors)?;
        );

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Type<OldCatalog<'own, AnnotatedT>, AnnotatedT> for PolicyType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_entity_name() -> &'static str {
        "PolicyType"
    }

    fn get_floria_group_id_prefix() -> &'static str {
        "policy"
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
            properties: self.properties.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.properties),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            targets: complete_type_names_from(&self.targets, context.parent_type, |entity| &entity.targets),
            triggers: self.triggers.clone(),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// PolicyTypeMap
//

/// Map of [PolicyType].
pub type PolicyTypeMap<AnnotatedT> = BTreeMap<Name, PolicyType<AnnotatedT>>;
