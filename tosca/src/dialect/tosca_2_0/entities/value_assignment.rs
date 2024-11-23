use super::{
    super::{super::super::grammar::*, data::*},
    attribute_definition::*,
    parameter_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
    std::collections::*,
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

    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// To Floria: variant, updater, and validator.
    pub fn to_floria(&self) -> (Option<Variant<WithoutAnnotations>>, Option<floria::Call>, Option<floria::Call>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let (variant, updater) = match &self.expression {
            Some(expression) => expression.to_floria(),
            None => (None, None),
        };

        let validator = match &self.validation {
            Some(validation) => {
                match validation {
                    Expression::Literal(_) => None, // TODO? can be either true or false only
                    Expression::Call(call) => Some(call.into()),
                }
            }
            None => None,
        };

        (variant, updater, validator)
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
        let (variant, updater, validator) = self.to_floria();
        let mut floria_property = floria::Property::new(variant, updater, validator, read_only);

        if let Some(data_type_name) = &self.data_type_name {
            floria_property.class_ids.add_tosca_type(data_type_name, directory, store.clone(), errors)?;
        }

        floria_property.metadata.set_tosca_entity(tosca_entity);
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
        _source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if let Some(parent_data_type_name) = &parent.data_type_name {
                match &self.data_type_name {
                    Some(data_type_name) => {
                        validate_type_name(
                            data_type_name,
                            parent_data_type_name,
                            catalog,
                            &mut errors.to_error_recipient(),
                        )?;
                    }

                    None => {
                        self.data_type_name = Some(parent_data_type_name.clone().in_scope(scope.clone()));
                    }
                }
            }

            // TODO: validate the type of literal expressions?

            if_none_clone(
                &mut self.description,
                &parent.description,
                &mut self.annotations,
                &parent.annotations,
                "description",
            );

            if_none_clone(
                &mut self.expression,
                &parent.expression,
                &mut self.annotations,
                &parent.annotations,
                "expression",
            );

            complete_validation(
                &mut self.validation,
                parent.validation.as_ref(),
                &mut self.annotations,
                &parent.annotations,
            );
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
        _source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            match &self.data_type_name {
                Some(data_type_name) => {
                    validate_type_name(data_type_name, &parent.type_name, catalog, &mut errors.to_error_recipient())?;
                }

                None => {
                    self.data_type_name = Some(parent.type_name.clone().in_scope(scope.clone()));
                }
            }

            if_none_clone(
                &mut self.description,
                &parent.description,
                &mut self.annotations,
                &parent.annotations,
                "description",
            );

            if self.expression.is_some() {
                if parent.value.is_some() {
                    // TODO: cannot override `value`
                    errors.to_error_recipient().give(MissingRequiredError::new("property".into(), "?".into()))?;
                }
            } else if parent.value.is_some() {
                self.expression = parent.value.clone();
            } else if parent.default.is_some() {
                self.expression = parent.default.clone();
            } else if parent.required {
                errors.to_error_recipient().give(MissingRequiredError::new("property".into(), "?".into()))?;
            }

            complete_validation(
                &mut self.validation,
                parent.validation.as_ref(),
                &mut self.annotations,
                &parent.annotations,
            );

            // TODO: validate the type of literal expressions?
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
        _source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            match &self.data_type_name {
                Some(data_type_name) => {
                    validate_type_name(data_type_name, &parent.type_name, catalog, &mut errors.to_error_recipient())?;
                }

                None => {
                    self.data_type_name = Some(parent.type_name.clone().in_scope(scope.clone()));
                }
            }

            if_none_clone(
                &mut self.description,
                &parent.description,
                &mut self.annotations,
                &parent.annotations,
                "description",
            );

            if self.expression.is_none() && parent.default.is_some() {
                self.expression = parent.default.clone();
            }

            complete_validation(
                &mut self.validation,
                parent.validation.as_ref(),
                &mut self.annotations,
                &parent.annotations,
            );

            // TODO: validate the type of literal expressions?
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
        _source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if let Some((parent, scope)) = parent {
            if let Some(parent_type_name) = &parent.type_name {
                match &self.data_type_name {
                    Some(data_type_name) => {
                        validate_type_name(
                            data_type_name,
                            parent_type_name,
                            catalog,
                            &mut errors.to_error_recipient(),
                        )?;
                    }

                    None => {
                        self.data_type_name = Some(parent_type_name.clone().in_scope(scope.clone()));
                    }
                }
            }

            if_none_clone(
                &mut self.description,
                &parent.description,
                &mut self.annotations,
                &parent.annotations,
                "description",
            );

            if self.expression.is_none() {
                if parent.value.is_some() {
                    self.expression = parent.value.clone();
                } else if parent.default.is_some() {
                    self.expression = parent.default.clone();
                } else if parent.required {
                    errors.to_error_recipient().give(MissingRequiredError::new("property".into(), "?".into()))?;
                }
            }

            complete_validation(
                &mut self.validation,
                parent.validation.as_ref(),
                &mut self.annotations,
                &parent.annotations,
            );

            // TODO: validate the type of literal expressions?
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
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: self.expression.clone(),
            validation: self.validation.clone(),
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
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: self.validation.clone(),
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
