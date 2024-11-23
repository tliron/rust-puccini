use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{cli::debug::*, std::zerocopy::*},
    std::collections::*,
};

//
// TriggerDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A trigger definition defines an event, condition, action tuple associated with a policy.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct TriggerDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description string for the trigger.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// The mandatory name of the event that activates the trigger's action.
    #[resolve(required)]
    #[debuggable(as(display), style(name), tag = tag::span)]
    pub event: ByteString,

    /// The optional condition that must evaluate to true in order for the trigger's action to be
    /// performed. Note: this is optional since sometimes the event occurrence itself is enough
    /// to trigger the action.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub condition: Variant<AnnotatedT>,

    /// The list of sequential activities to be performed when the event is triggered, and the
    /// condition is met (i.e., evaluates to true).
    #[resolve]
    #[debuggable(iter(item), as(debuggable))]
    pub action: Vec<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<TriggerDefinition<AnnotatedT>> for TriggerDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _parent: &Self,
        _depot: &mut Depot,
        _source_id: &SourceID,
        _scope: &Scope,
        _errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        // TODO
        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<TriggerDefinition<AnnotatedT>> for TriggerDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, _scope: &Scope) -> Self {
        self.clone()
    }
}

//
// TriggerDefinitions
//

/// Map of [TriggerDefinition].
pub type TriggerDefinitions<AnnotatedT> = BTreeMap<ByteString, TriggerDefinition<AnnotatedT>>;
