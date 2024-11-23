use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        attribute_definition::*,
        capability_definition::*,
        node_type::*,
        property_definition::*,
        value_assignments::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
        unwrap_or_give_and_return,
    },
    std::collections::*,
};

//
// CapabilityAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability assignment allows node template authors to assign values to properties and
/// attributes for a capability definition that is part of the node template's type definition.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Map of property assignments.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// Map of attribute assignments.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional list of directive values to provide processing instructions to orchestrators
    /// and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// To Floria.
    pub fn to_floria<ErrorRecipientT>(
        &self,
        _floria_node_template: &mut floria::NodeTemplate,
        _errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        // TODO
        Ok(())
    }

    /// Compile to Floria.
    pub fn compile_to_floria<StoreT, ErrorRecipientT>(
        &self,
        context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
        capability_name: &str,
        node_template_id: floria::ID,
        node_type: &NodeType<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut floria_node_template = floria::NodeTemplate::new_for(
            context.floria_prefix.clone(),
            capability_name.into(),
            Some(node_template_id),
        );

        floria_node_template.template.metadata.set_tosca_entity("CapabilityAssignment");
        floria_node_template.template.metadata.set_tosca_directives(&self.directives);

        match node_type.capabilities.get(capability_name) {
            Some(capability_definition) => {
                // Properties
                floria_node_template.template.property_templates = self.properties.compile_to_floria_as_properties(
                    &capability_definition.properties,
                    context.catalog,
                    context.index,
                    errors,
                )?;

                // Attributes
                floria_node_template.template.property_templates.extend(
                    self.attributes.compile_to_floria_as_attributes(
                        &capability_definition.attributes,
                        context.catalog,
                        context.index,
                        errors,
                    )?,
                );

                context.catalog.capability_types.add_floria_group_ids(
                    &mut floria_node_template.template.group_ids,
                    &"capability".into(),
                    context.index.index.get(&capability_definition.type_name).unwrap(),
                );
            }

            None => tracing::warn!("capability definition not found: {}", capability_name),
        }

        let id = floria_node_template.template.id.clone();
        unwrap_or_give_and_return!(context.store.add_node_template(floria_node_template), errors, Ok(None));
        Ok(Some(id))
    }
}

impl<AnnotatedT> Subentity<CapabilityDefinition<AnnotatedT>> for CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &CapabilityDefinition<AnnotatedT>,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "attributes",
            complete_map(&mut self.attributes, &parent.attributes, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<CapabilityAssignment<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> CapabilityAssignment<AnnotatedT> {
        CapabilityAssignment {
            properties: self.properties.into_scoped(scope),
            attributes: self.attributes.into_scoped(scope),
            annotations: clone_struct_annotations(&self.annotations, &["properties", "attributes"]),
            ..Default::default()
        }
    }
}

impl<'own, AnnotatedT> Assignment<CapabilityDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_assignment_entity_name() -> &'static str {
        "capability"
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, CapabilityDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            properties: self.properties.complete_as_properties(&context.definition.properties, errors)?,
            attributes: self.attributes.complete_as_attributes(&context.definition.attributes, errors)?,
            directives: Default::default(),
            annotations: Default::default(),
        })
    }

    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, CapabilityDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            properties: context.definition.properties.to_assignments(errors)?,
            attributes: context.definition.attributes.to_assignments(),
            directives: Default::default(),
            annotations: Default::default(),
        })
    }
}

//
// CapabilityAssignments
//

/// Map of [CapabilityAssignment].
pub type CapabilityAssignments<AnnotatedT> = BTreeMap<ByteString, CapabilityAssignment<AnnotatedT>>;
