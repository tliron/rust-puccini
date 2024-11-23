use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        attribute_definition::*,
        interface_assignment::*,
        property_definition::*,
        relationship_definition::*,
        value_assignments::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::debug::*, std::error::*},
};

//
// RelationshipAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The relationship keyname in a requirement assignment typically specifies a relationship
/// assignment that provides information needed by TOSCA Orchestrators to construct a relationship
/// to the TOSCA node that is the target of the requirement.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional keyname used to provide the name of the relationship type for the requirement
    /// assignment's relationship.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// An optional map of property assignments for the relationship.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute assignments for the relationship.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the corresponding interface definitions in the
    /// relationship type.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RelationshipDefinition<AnnotatedT>> for RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &RelationshipDefinition<AnnotatedT>,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "attributes",
            complete_map(&mut self.attributes, &parent.attributes, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "interfaces",
            complete_map(&mut self.interfaces, &parent.interfaces, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<RelationshipAssignment<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> RelationshipAssignment<AnnotatedT> {
        RelationshipAssignment {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            properties: self.properties.into_scoped(scope),
            attributes: self.attributes.into_scoped(scope),
            interfaces: self.interfaces.into_scoped(scope),
            ..Default::default()
        }
    }
}

impl<'own, AnnotatedT> Assignment<RelationshipDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_assignment_entity_name() -> &'static str {
        "relationship"
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, RelationshipDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.definition.type_name.clone(),
            properties: self.properties.complete_as_properties(&context.definition.properties, errors)?,
            attributes: self.attributes.complete_as_attributes(&context.definition.attributes, errors)?,
            interfaces: self.interfaces.complete(
                AssignmentsAsMapCompleteContext {
                    definitions: &context.definition.interfaces,
                    catalog: context.catalog,
                },
                errors,
            )?,
            annotations: Default::default(),
        })
    }

    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, RelationshipDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.definition.type_name.clone(),
            properties: context.definition.properties.to_assignments(errors)?,
            attributes: context.definition.attributes.to_assignments(),
            interfaces: AssignmentsAsMap::from_definitions(
                AssignmentsAsMapFromDefinitionsContext {
                    definitions: &context.definition.interfaces,
                    catalog: context.catalog,
                },
                errors,
            )?,
            annotations: Default::default(),
        })
    }
}
