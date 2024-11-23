use super::{
    super::{super::super::grammar::*, dialect::*},
    attribute_definition::*,
    interface_definition::*,
    property_definition::*,
    relationship_type::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
};

//
// RelationshipDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The relationship keyname in a requirement definition specifies a relationship definition that
/// provides information needed by TOSCA Orchestrators to construct a relationship to the TOSCA
/// node that contains the matching target capability.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory keyname used to provide the name of the relationship type used for the
    /// relationship.
    #[resolve(single, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description of the relationship definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	An optional map of property refinements for the relationship definition. The referred
    /// properties must have been defined in the relationship type definition referred by the type
    /// keyname. New properties may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the relationship definition. The referred
    /// attributes must have been defined in the relationship type definition referred by the
    /// type keyname. New attributes may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// The optional keyname used to define interface refinements for interfaces defined by the
    /// relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RelationshipDefinition<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<(&Self, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        if let Some((parent, _scope)) = &parent {
            if self.type_name.is_empty() && !parent.type_name.is_empty() {
                self.type_name = parent.type_name.clone();
            } else {
                validate_type_name(&self.type_name, &parent.type_name, catalog, errors)?;
            }
        }

        let relationship_type =
            get_complete_entity!(RELATIONSHIP_TYPE, RelationshipType, self, type_name, catalog, source_id, errors);

        complete_map_for_field!(properties, self, relationship_type, catalog, source_id, errors);
        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, relationship_type, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, parent, catalog, source_id, errors);
        complete_map_for_field!(interfaces, self, relationship_type, catalog, source_id, errors);
        complete_map_for_field!(interfaces, self, parent, catalog, source_id, errors);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<RelationshipDefinition<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            properties: self.properties.convert_into_scope(scope),
            attributes: self.attributes.convert_into_scope(scope),
            interfaces: self.interfaces.convert_into_scope(scope),
            annotations: self.annotations.clone(),
        }
    }
}
