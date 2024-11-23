use super::{
    super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
    data_type::*,
    value_assignment::*,
    value_assignments::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
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
    pub default: Option<ValueAssignment<AnnotatedT>>,

    /// An optional key that may provide a fixed value to be used. A property that has a fixed
    /// value provided (as part of a definition or refinement) cannot be subject to a further
    /// refinement or assignment. That is, a fixed value cannot be changed.
    #[resolve]
    #[depict(option, as(depict))]
    pub value: Option<ValueAssignment<AnnotatedT>>,

    /// The optional validation clause for the property.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<ValueAssignment<AnnotatedT>>,

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
        parent: &Self,
        depot: &mut Depot,
        source_id: &SourceID,
        _scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        if let Some(_data_type) = depot
            .get_complete_entity::<DataType<AnnotatedT>, _, _>(
                DATA_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            //let scope = &self.type_name.scope;

            // merge "validation" field ($and ?)
            //
            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<PropertyDefinition<AnnotatedT>> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
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

impl<AnnotatedT> PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// To assignment.
    pub fn to_assignment<ErrorRecipientT>(
        &self,
        property_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignment<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(match &self.value {
            Some(value) => value.clone(),

            None => match &self.default {
                Some(default) => default.clone(),

                None => {
                    if self.required {
                        errors.give(MissingRequiredError::new("property".into(), property_name.into()))?;
                    }

                    Default::default()
                }
            },
        })
    }
}

impl<'own, AnnotatedT> Definition<DataType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, DataType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.type_name.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.clone(),
            value: self.value.clone(),
            validation: complete_clone(&self.validation, &context.type_.validation),
            key_schema: complete_clone(&self.key_schema, &context.type_.key_schema),
            entry_schema: complete_clone(&self.entry_schema, &context.type_.entry_schema),
            annotations: self.annotations.clone(),
        })
    }

    fn derive<ErrorRecipientT>(
        &self,
        context: DefinitionDeriveContext<'_, Self, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: self.type_name.clone(),
            description: complete_clone(&self.description, &context.parent_definition.description),
            metadata: self.metadata.clone(),
            required: self.required,
            default: complete_clone(&self.default, &context.parent_definition.default),
            value: complete_clone(&self.value, &context.parent_definition.value),
            validation: complete_clone(&self.validation, &context.parent_definition.validation),
            key_schema: complete_clone(&self.key_schema, &context.parent_definition.key_schema),
            entry_schema: complete_clone(&self.entry_schema, &context.parent_definition.entry_schema),
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.clone().in_scope(scope.clone());
    }
}

//
// PropertyDefinitions
//

/// Map of [PropertyDefinition].
pub type PropertyDefinitions<AnnotatedT> = BTreeMap<ByteString, PropertyDefinition<AnnotatedT>>;

//
// PropertyDefinitionsUtilities
//

/// Utilities for [PropertyDefinitions].
pub trait PropertyDefinitionsUtilities<AnnotatedT> {
    /// To property assignments.
    fn to_assignments<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl<AnnotatedT> PropertyDefinitionsUtilities<AnnotatedT> for PropertyDefinitions<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_assignments<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut assignments = BTreeMap::default();

        for (property_name, property_definition) in self {
            let assignment = property_definition.to_assignment(property_name, errors)?;
            assignments.insert(property_name.clone(), assignment);
        }

        Ok(assignments)
    }
}
