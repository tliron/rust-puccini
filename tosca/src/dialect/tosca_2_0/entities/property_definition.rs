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
    smart_default::*,
    std::collections::*,
};

//
// PropertyDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A property definition defines a named, typed value and related data that can be associated with
/// an entity defined in this specification (e.g., node types, relationship types, capability types,
/// etc.). Properties are used by template authors to provide configuration values to TOSCA entities
/// that indicate their desired state when they are instantiated. The value of a property can be
/// retrieved using the $get_property function within TOSCA service templates.
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the property.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the property.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional key that declares a property as required (true) or not (false). Defaults to
    /// true.
    #[default(true)]
    #[resolve]
    #[depict(style(name))]
    pub required: bool,

    /// An optional key that may provide a value to be used as a default if not provided by another
    /// means. The default keyname SHALL NOT be defined when property is not required (i.e. the
    /// value of the required keyname is false).
    #[resolve]
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// An optional key that may provide a fixed value to be used. A property that has a fixed
    /// value provided (as part of a definition or refinement) cannot be subject to a further
    /// refinement or assignment. That is, a fixed value cannot be changed.
    #[resolve]
    #[depict(option, as(depict))]
    pub value: Option<Expression<AnnotatedT>>,

    /// The optional validation clause for the property.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The schema definition for the keys used to identify entries in properties of type map (or
    /// types that derive from map). If not specified, the key_schema defaults to string. For
    /// properties of type other than map, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<Variant<AnnotatedT>>,

    /// The schema definition for the entries in properties of collection types such as list, map,
    /// or types that derive from list or map. If the property type is a collection type,
    /// entry_schema is mandatory. For other types, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for PropertyDefinition<AnnotatedT>
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

            // if requirement=true, default cannot be Some

            // if value=None, default cannot be Some

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<PropertyDefinition<AnnotatedT>> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.clone(),
            value: self.value.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.clone(),
            entry_schema: self.entry_schema.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

//
// PropertyDefinitions
//

/// Map of [PropertyDefinition].
pub type PropertyDefinitions<AnnotatedT> = BTreeMap<ByteString, PropertyDefinition<AnnotatedT>>;
