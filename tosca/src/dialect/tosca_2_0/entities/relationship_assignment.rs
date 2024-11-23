use super::{
    super::super::super::grammar::*, interface_assignment::*, relationship_definition::*, value_assignment::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
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
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional keyname used to provide the name of the relationship type for the requirement
    /// assignment's relationship.
    #[resolve(required, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// An optional map of property assignments for the relationship.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute assignments for the relationship.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the corresponding interface definitions in the
    /// relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RelationshipDefinition<AnnotatedT>> for RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<(&RelationshipDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        if let Some((parent, _scope)) = &parent {
            validate_type_name(&self.type_name, &parent.type_name, catalog, errors)?;
        }

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, parent, catalog, source_id, errors);
        complete_map_for_field!(interfaces, self, parent, catalog, source_id, errors);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<RelationshipAssignment<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> RelationshipAssignment<AnnotatedT> {
        RelationshipAssignment {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            properties: self.properties.convert_into_scope(scope),
            attributes: self.attributes.convert_into_scope(scope),
            interfaces: self.interfaces.convert_into_scope(scope),
            annotations: clone_struct_annotations(
                &self.annotations,
                &["type_name", "properties", "attributes", "interfaces"],
            ),
            ..Default::default()
        }
    }
}
