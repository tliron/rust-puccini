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
// ParameterDefinition
//

// Copied from PropertyDefinition, except that "type" is not required

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A parameter definition defines a named, typed value and related data that may be used to exchange
/// values between the TOSCA orchestrator and the external world.
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The data type of the parameter. While this keyname is mandatory for a TOSCA Property definition,
    /// it is not mandatory for a TOSCA parameter definition.
    #[resolve(key = "type")]
    #[depict(option, as(display), style(name))]
    pub type_name: Option<FullName>,

    /// The type-compatible value to assign to the parameter. Parameter values may be provided as
    /// the result from the evaluation of an expression or a function. May only be defined for
    /// outgoing parameters. Mutually exclusive with the mapping keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub value: Option<ValueAssignment<AnnotatedT>>,

    /// A mapping that specifies the node or relationship attribute into which the returned output
    /// value must be stored. May only be defined for incoming parameters. Mutually exclusive with
    /// the value keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub mapping: Option<Variant<AnnotatedT>>,

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

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ParameterDefinition<AnnotatedT>
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

        if let Some(type_name) = &self.type_name {
            if let Some(parent_type_name) = &parent.type_name {
                validate_type_name(type_name, parent_type_name, depot, errors)?;
            }

            if let Some(_data_type) = depot
                .get_complete_entity::<DataType<AnnotatedT>, _, _>(
                    DATA_TYPE,
                    type_name,
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
        }

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ParameterDefinition<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().map(|type_name| type_name.in_scope(scope.clone())),
            value: self.value.clone(),
            mapping: self.mapping.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.clone(),
            entry_schema: self.entry_schema.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> ParameterDefinition<AnnotatedT>
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
                        errors.give(MissingRequiredError::new("parameter".into(), property_name.into()))?;
                        // TODO: annotations
                    }

                    Default::default()
                }
            },
        })
    }
}

impl<'own, AnnotatedT> Definition<DataType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        self.type_name.as_ref()
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
            type_name: Some(context.type_name.clone()),
            value: self.value.clone(),
            mapping: self.mapping.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.clone(),
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
            type_name: complete_clone(&self.type_name, &context.parent_definition.type_name),
            value: complete_clone(&self.value, &context.parent_definition.value),
            mapping: complete_clone(&self.mapping, &context.parent_definition.mapping),
            description: complete_clone(&self.description, &context.parent_definition.description),
            metadata: self.metadata.clone(),
            required: self.required,
            default: complete_clone(&self.default, &context.parent_definition.default),
            validation: complete_clone(&self.validation, &context.parent_definition.validation),
            key_schema: complete_clone(&self.key_schema, &context.parent_definition.key_schema),
            entry_schema: complete_clone(&self.entry_schema, &context.parent_definition.entry_schema),
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.as_ref().map(|name| name.clone().in_scope(scope.clone()));
    }
}

//
// ParameterDefinitions
//

/// Parameter definitions.
pub trait ParameterDefinitions<AnnotatedT> {
    /// To parameter assignments.
    fn to_assignments<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

//
// ParameterDefinitionMap
//

/// Map of [ParameterDefinition].
pub type ParameterDefinitionMap<AnnotatedT> = BTreeMap<ByteString, ParameterDefinition<AnnotatedT>>;

impl<AnnotatedT> ParameterDefinitions<AnnotatedT> for ParameterDefinitionMap<AnnotatedT>
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

        for (parameter_name, parameter_definition) in self {
            let assignment = parameter_definition.to_assignment(parameter_name, errors)?;
            assignments.insert(parameter_name.clone(), assignment);
        }

        Ok(assignments)
    }
}
