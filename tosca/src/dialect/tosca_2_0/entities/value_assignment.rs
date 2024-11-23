use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    data_type::*,
    data_type_kind::*,
    parameter_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::{collections::*, mem::*},
};

//
// ValueAssignment
//

/// Value assignment.
///
/// For properties, attributes, and parameters.
#[derive(Clone, Debug, Default, Depict)]
pub struct ValueAssignment<AnnotatedT> {
    /// Data type name.
    #[depict(option, as(depict))]
    pub data_type_name: Option<FullName>,

    /// Data type kind.
    #[depict(option, style(symbol))]
    pub data_type_kind: Option<DataTypeKind>,

    /// Description.
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Metadata.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// Expression.
    #[depict(option, as(depict))]
    pub expression: Option<Expression<AnnotatedT>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// Applied data type.
    #[depict(style(symbol))]
    pub applied: bool,

    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// Apply data type.
    ///
    /// TODO: process "default" and "value" too?
    pub fn apply_data_type<ErrorRecipientT>(
        &mut self,
        data_type: &DataType<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        if self.data_type_kind.is_none() && data_type.kind.is_some() {
            self.data_type_kind = data_type.kind.clone();
        }

        if !self.applied {
            match take(&mut self.expression) {
                Some(Expression::Literal(literal)) => {
                    let variant = unwrap_or_give_and_return!(data_type.apply(literal), errors, Ok(()));
                    self.expression = Some(variant.into());
                }

                expression => self.expression = expression,
            }

            self.applied = true;
        }

        Ok(())
    }

    /// To Floria property variant, parser, updater, and validator.
    pub fn to_floria_property_fields(
        &self,
    ) -> (Option<Variant<WithoutAnnotations>>, Option<floria::Call>, Option<floria::Call>, Option<floria::Call>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let (variant, updater) = match &self.expression {
            Some(expression) => expression.to_floria_property_fields(),
            None => (None, None),
        };

        let validator = match &self.validation {
            Some(validation) => validation.to_floria_property_validator(),
            None => None,
        };

        (variant, None, updater, validator)
    }

    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        tosca_entity: &str,
        read_only: bool,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<floria::Property, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let (variant, parser, updater, validator) = self.to_floria_property_fields();
        let mut floria_property = floria::Property::new(variant, updater, parser, validator, read_only);

        if let Some(data_type_name) = &self.data_type_name {
            floria_property.class_ids.add_tosca_type(data_type_name, directory, store.clone(), errors)?;
        }

        if let Some(data_type_kind) = self.data_type_kind {
            floria_property.metadata.set_tosca_data(&data_type_kind.to_string());
        }

        floria_property.metadata.set_tosca_entity(DIALECT_ID, tosca_entity);
        floria_property.metadata.set_tosca_description(self.description.as_ref());
        floria_property.metadata.merge_tosca_metadata(&self.metadata);

        Ok(floria_property)
    }
}

// Used by ArtifactDefinition
impl<AnnotatedT> Subentity<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: Option<(&Self, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if_none_else!(
                data_type_name,
                self,
                parent,
                parent.data_type_name.as_ref().map(|data_type_name| data_type_name.clone().in_scope(scope.clone()))
            );

            if_none_clone!(description, self, parent);
            if_none_clone!(expression, self, parent);
            complete_validation!(self, parent);

            let errors = &mut errors.to_error_recipient();

            if let Some(data_type_name) = &self.data_type_name
                && let Some(data_type) = catalog
                    .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type_name, source_id, errors)?
                    .cloned()
            {
                if let Some(data_type_name) = &parent.data_type_name {
                    validate_type(&data_type, data_type_name, catalog, errors)?;
                }

                self.apply_data_type(&data_type, &mut errors.into_annotated())
                    .map_err(|error| error.into_annotated())?;
            }
        }

        Ok(())
    }
}

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: Option<(&PropertyDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if_none_else!(data_type_name, self, parent, Some(parent.type_name.clone().in_scope(scope.clone())));
            if_none_clone!(description, self, parent);
            complete_validation!(self, parent);

            let errors = &mut errors.to_error_recipient();

            if self.expression.is_some() {
                if parent.value.is_some() {
                    // TODO: cannot override `value`
                    errors.give(MissingRequiredError::new("property".into(), None))?;
                }
            } else if parent.value.is_some() {
                self.expression = parent.value.clone();
            } else if parent.default.is_some() {
                self.expression = parent.default.clone();
            } else if parent.required {
                errors.give(MissingRequiredError::new("property".into(), None))?;
            }

            if let Some(data_type_name) = &self.data_type_name
                && let Some(data_type) = catalog
                    .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type_name, source_id, errors)?
                    .cloned()
            {
                validate_type(&data_type, &parent.type_name, catalog, errors)?;
                self.apply_data_type(&data_type, &mut errors.into_annotated())
                    .map_err(|error| error.into_annotated())?;
            }
        }

        Ok(())
    }
}

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: Option<(&AttributeDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if_none_else!(data_type_name, self, parent, Some(parent.type_name.clone().in_scope(scope.clone())));
            if_none_clone!(description, self, parent);
            complete_validation!(self, parent);

            if self.expression.is_none() && parent.default.is_some() {
                self.expression = parent.default.clone();
            }

            let errors = &mut errors.to_error_recipient();

            if let Some(data_type_name) = &self.data_type_name
                && let Some(data_type) = catalog
                    .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type_name, source_id, errors)?
                    .cloned()
            {
                validate_type(&data_type, &parent.type_name, catalog, errors)?;
                self.apply_data_type(&data_type, &mut errors.into_annotated())
                    .map_err(|error| error.into_annotated())?;
            }
        }

        Ok(())
    }
}

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: Option<(&ParameterDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if_none_else!(
                data_type_name,
                self,
                parent,
                parent.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone()))
            );

            if_none_clone!(description, self, parent);
            complete_validation!(self, parent);

            let errors = &mut errors.to_error_recipient();

            if self.expression.is_none() {
                if parent.value.is_some() {
                    self.expression = parent.value.clone();
                } else if parent.default.is_some() {
                    self.expression = parent.default.clone();
                } else if parent.required {
                    errors.give(MissingRequiredError::new("property".into(), None))?;
                }
            }

            if let Some(data_type_name) = &self.data_type_name
                && let Some(data_type) = catalog
                    .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type_name, source_id, errors)?
                    .cloned()
            {
                if let Some(type_name) = &parent.type_name {
                    validate_type(&data_type, type_name, catalog, errors)?;
                }

                self.apply_data_type(&data_type, &mut errors.into_annotated())
                    .map_err(|error| error.into_annotated())?;
            }
        }

        Ok(())
    }
}

// For ArtifactAssignment and ArtifactDefinition
impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            data_type_name: self
                .data_type_name
                .as_ref()
                .map(|data_type_name| data_type_name.clone().in_scope(scope.clone())),
            data_type_kind: self.data_type_kind.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: self.expression.clone(),
            validation: self.validation.clone(),
            applied: self.applied,
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            data_type_name: Some(self.type_name.clone().in_scope(scope.clone())),
            data_type_kind: None,
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: self.validation.clone(),
            applied: false,
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            data_type_name: Some(self.type_name.clone().in_scope(scope.clone())),
            data_type_kind: None,
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: self.validation.clone(),
            applied: false,
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            data_type_name: self.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone())),
            data_type_kind: None,
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: self.validation.clone(),
            applied: false,
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let value: Option<Expression<_>> = self.resolve_with_errors(errors)?;
        Ok(value.map(|value| value.into()))
    }
}

impl<AnnotatedT> From<Expression<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(expression: Expression<AnnotatedT>) -> Self {
        Self { expression: Some(expression), ..Default::default() }
    }
}

//
// ValueAssignments
//

/// Map of [ValueAssignment].
pub type ValueAssignments<AnnotatedT> = BTreeMap<ByteString, ValueAssignment<AnnotatedT>>;
