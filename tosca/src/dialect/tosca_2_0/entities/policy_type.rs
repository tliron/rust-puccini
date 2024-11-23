use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
    trigger_definition::*,
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

impl_type_entity!(PolicyType);

impl<AnnotatedT> Entity for PolicyType<AnnotatedT>
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

        let parent = get_complete_parent!(POLICY_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(triggers, self, parent, catalog, source_id, errors);

        if let Some((parent, scope)) = parent {
            errors_with_field_annotations!(
                errors, self, "targets",
                complete_types(&mut self.targets, &parent.targets, catalog, source_id, scope, errors)?;
            );
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// PolicyTypes
//

/// Map of [PolicyType].
pub type PolicyTypes<AnnotatedT> = BTreeMap<Name, PolicyType<AnnotatedT>>;
