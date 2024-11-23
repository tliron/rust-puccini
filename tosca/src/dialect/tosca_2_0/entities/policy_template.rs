use super::{
    super::{super::super::grammar::*, dialect::*},
    policy_type::*,
    trigger_definition::*,
    value_assignment::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
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
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PolicyTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the policy type the policy definition is based upon.
    #[resolve(required, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the policy definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property value assignments for the policy definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional list of valid node templates or Groups the Policy can be applied to.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub targets: Vec<Name>,

    /// An optional map of trigger definitions to invoke when the policy is applied by an
    /// orchestrator against the associated TOSCA entity. These triggers apply in addition to the
    /// triggers defined in the policy type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub triggers: TriggerDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
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
        catalog: &mut Catalog,
        source_id: &SourceID,
        _callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let policy_type = get_complete_entity!(POLICY_TYPE, PolicyType, self, type_name, catalog, source_id, errors);

        complete_map_for_field!(properties, self, policy_type, catalog, source_id, errors);
        complete_map_for_field!(triggers, self, policy_type, catalog, source_id, errors);

        if let Some((policy_type, _scope)) = policy_type {
            validate_entities_types(&self.targets, &policy_type.targets, catalog, errors)?;
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// PolicyTemplates
//

/// Vector of [PolicyTemplate].
pub type PolicyTemplates<AnnotatedT> = Vec<PolicyTemplate<AnnotatedT>>;
