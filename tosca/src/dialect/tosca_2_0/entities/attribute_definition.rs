use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// AttributeDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An attribute definition defines a named, typed value that can be associated with an entity defined
/// in this specification (e.g., a node, relationship or capability type). Specifically, it is used
/// to expose the actual state of some a TOSCA entity after it has been deployed and instantiated (as
/// set by the TOSCA orchestrator).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the attribute.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the attribute.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional key that may provide a value to be used as a default if not provided by another
    /// means. This value SHALL be type compatible with the type declared by the attribute
    /// definition's type keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// The optional validation clause for the attribute.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The schema definition for the keys used to identify entries in attributes of type TOSCA map
    /// (or types that derive from map). If not specified, the key_schema defaults to string. For
    /// attributes of type other than map, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<Variant<AnnotatedT>>,

    /// The schema definition for the entries in attributes of collection types such as list, map,
    /// or types that derive from list or map) If the attribute type is a collection type,
    /// entry_schema is mandatory. For other types, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for AttributeDefinition<AnnotatedT>
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

        self.validation.finalize_validation(None);

        let data_type = get_complete_entity!(DATA_TYPE, DataType, self, type_name, catalog, source_id, errors);

        if let Some((data_type, _scope)) = data_type {
            complete_validation!(self, data_type);

            //let scope = &self.type_name.scope;

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<AttributeDefinition<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            default: self.default.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.clone(),
            entry_schema: self.entry_schema.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

//
// AttributeDefinitions
//

/// Map of [AttributeDefinition].
pub type AttributeDefinitions<AnnotatedT> = BTreeMap<ByteString, AttributeDefinition<AnnotatedT>>;
