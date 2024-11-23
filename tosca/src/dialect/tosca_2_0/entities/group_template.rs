use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        group_type::*,
        value_assignments::*,
    },
    crate::{errors_with_field_annotations, impl_entity},
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
// GroupTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// Collections of nodes in a service template may be grouped together using a group definition in
/// that same service template. A group definition defines a logical grouping of node templates for
/// purposes of uniform application of policies.
///
/// Puccini note: Though this is called a "definition" in the TOSCA spec, it is actually used as a
/// template.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct GroupTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the group type the group definition is based upon.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// The optional description for the group definition.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property value assignments for the group definition.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute value assignments for the group definition.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// The optional list of one or more node template names that are members of this group
    /// definition.
    #[resolve]
    #[debuggable(iter(item), as(debuggable), tag = tag::span)]
    pub members: Vec<Name>,

    #[debuggable(skip)]
    id: ID,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for GroupTemplate<AnnotatedT>
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

        let Some(group_type) = depot
            .get_complete_entity_next::<GroupType<_>, _, _>(GROUP_TYPE, &self.type_name, source_id, callstack, errors)?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &self.type_name.scope;

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &group_type.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "attributes",
            complete_map(&mut self.attributes, &group_type.attributes, depot, source_id, scope, errors)?;
        );

        validate_entities_types(&self.members, &group_type.members, depot, errors)?;

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl_entity!(GroupTemplate);

impl<'own, AnnotatedT> Template<GroupType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for GroupTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> &FullName {
        &self.type_name
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: TemplateCompleteContext<'_, GroupType<AnnotatedT>, OldCatalog<'_, AnnotatedT>>,
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
            attributes: self.attributes.complete_as_attributes(&context.type_.attributes, errors)?,
            // TODO: conform with group_type types
            members: self.members.clone(),
            id: self.id.clone(),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// GroupTemplates
//

/// Map of [GroupTemplate].
pub type GroupTemplates<AnnotatedT> = BTreeMap<Name, GroupTemplate<AnnotatedT>>;
