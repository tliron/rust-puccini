use super::{
    super::{super::super::grammar::*, dialect::*},
    attribute_definition::*,
    capability_type::*,
    property_definition::*,
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
// CapabilityDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability definition defines a typed set of data that a node can expose and that is used to
/// describe a relevant feature of the component described by the node that can be used to fulfill
/// a requirement exposed by another node. A capability is defined as part of a node type
/// definition and may be refined during node type derivation.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the capability type on which this capability definition is based.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description of the Capability definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property refinements for the capability definition. The referred
    /// properties must have been defined in the capability type definition referred to by the
    /// type keyname. New properties may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the capability definition. The referred
    /// attributes must have been defined in the capability type definition referred by the type
    /// keyname. New attributes may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as valid
    /// sources of any relationship established to the declared capability type. If undefined, all
    /// node types are valid sources. If valid_source_node_types is defined in the capability type,
    /// each element in this list must either be or derived from an element in the list defined in
    /// the type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported as
    /// valid types of any relationship established to the declared capability type. If undefined,
    /// all relationship types are valid. If valid_relationship_types is defined in the capability
    /// type, each element in this list must either be or derived from an element in the list
    /// defined in the type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<CapabilityDefinition<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
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

        let capability_type =
            get_complete_entity!(CAPABILITY_TYPE, CapabilityType, self, type_name, catalog, source_id, errors);

        complete_map_for_field!(properties, self, capability_type, catalog, source_id, errors);
        complete_map_for_field!(attributes, self, capability_type, catalog, source_id, errors);

        if let Some((capability_type, scope)) = &capability_type {
            errors_with_field_annotations!(
                errors, self, "valid_source_node_types",
                complete_types(
                    &mut self.valid_source_node_types,
                    &capability_type.valid_source_node_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );

            errors_with_field_annotations!(
                errors, self, "valid_relationship_types",
                complete_types(
                    &mut self.valid_relationship_types,
                    &capability_type.valid_relationship_types,
                    catalog,
                    source_id,
                    scope,
                    errors,
                )?;
            );
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<CapabilityDefinition<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
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
            valid_source_node_types: self.valid_source_node_types.convert_into_scope(scope),
            valid_relationship_types: self.valid_relationship_types.convert_into_scope(scope),
            annotations: self.annotations.clone(),
        }
    }
}

//
// CapabilityDefinitions
//

/// Map of [CapabilityDefinition].
pub type CapabilityDefinitions<AnnotatedT> = BTreeMap<ByteString, CapabilityDefinition<AnnotatedT>>;
