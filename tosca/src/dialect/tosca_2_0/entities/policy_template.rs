use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        policy_type::*,
        trigger_definition::*,
        value_assignments::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::debug::*,
        std::{error::*, zerocopy::*},
    },
};

//
// PolicyTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A policy definition defines a policy that can be associated with a TOSCA service or top-level
/// entity definition (e.g., group definition, node template, etc.).
///
/// Puccini note: Though this is called a "definition" in the TOSCA spec, it is actually used as a
/// template.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PolicyTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the policy type the policy definition is based upon.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// The optional description for the policy definition.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property value assignments for the policy definition.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional list of valid node templates or Groups the Policy can be applied to.
    #[resolve]
    #[debuggable(iter(item), as(debuggable), tag = tag::span)]
    pub targets: Vec<Name>,

    /// An optional map of trigger definitions to invoke when the policy is applied by an
    /// orchestrator against the associated TOSCA entity. These triggers apply in addition to the
    /// triggers defined in the policy type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub triggers: TriggerDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for PolicyTemplate<AnnotatedT>
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
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let Some(policy_type) = depot
            .get_complete_entity_next::<PolicyType<_>, _, _>(
                POLICY_TYPE,
                &self.type_name,
                source_id,
                callstack,
                errors,
            )?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &self.type_name.scope;

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &policy_type.properties, depot, source_id, scope, errors)?;
        );

        validate_entities_types(&self.targets, &policy_type.targets, depot, errors)?;

        errors_with_field_annotations!(
            errors, self, "triggers",
            complete_map(&mut self.triggers, &policy_type.triggers, depot, source_id, scope, errors)?;
        );

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Template<PolicyType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for PolicyTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> &FullName {
        &self.type_name
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: TemplateCompleteContext<'_, PolicyType<AnnotatedT>, OldCatalog<'_, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: self.type_name.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            properties: self.properties.complete_as_properties(&context.type_.properties, errors)?,
            // TODO: conform with types
            targets: self.targets.clone(),
            // TODO
            triggers: self.triggers.clone(),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// PolicyTemplateVector
//

/// Vector of [PolicyTemplate].
pub type PolicyTemplateVector<AnnotatedT> = Vec<PolicyTemplate<AnnotatedT>>;
