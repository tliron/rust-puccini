use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    data_type::*,
    parameter_definition::*,
    property_definition::*,
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
// ValueAssignment
//

/// Value assignment.
///
/// For properties, attributes, and parameters.
#[derive(Clone, Debug, Default, Depict)]
pub struct ValueAssignment<AnnotatedT> {
    /// Expression.
    #[depict(option, as(depict))]
    pub expression: Option<Expression<AnnotatedT>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// Data type.
    #[depict(option, as(depict))]
    pub data_type: Option<FullName>,

    /// Schema. TODO?
    #[depict(option, as(depict))]
    pub schema: Option<Schema<AnnotatedT>>,

    /// Metadata.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// Description.
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// Floria property value, preparer, and updater.
    pub fn floria_property_fields(
        &self,
    ) -> (Option<floria::Expression>, Option<floria::Expression>, Option<floria::Expression>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let (value, updater) = self
            .expression
            .as_ref()
            .map(|expression| expression.clone().into_floria_property_fields())
            .unwrap_or_default();

        let preparer = self.validation.as_ref().map(|validation| validation.clone().into());

        (value, preparer, updater)
    }

    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        tosca_entity: &'static str,
        read_only: bool,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<floria::Property, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let (value, preparer, updater) = self.floria_property_fields();
        let mut floria_property = floria::Property::new(read_only, preparer, updater, value);

        floria_property.metadata.set_tosca_entity_static(DIALECT_ID, tosca_entity);
        floria_property.metadata.set_tosca_description(self.description.as_ref());
        floria_property.metadata.merge_tosca_metadata(&self.metadata);

        if let Some(data_type) = &self.data_type {
            floria_property.class_ids.add_tosca_type(data_type, directory, store.clone(), errors)?;
        }

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
        _name: Option<ByteString>,
        parent: Option<(&Self, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((parent, scope)) = parent else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if_none_clone!(expression, self, parent);
        complete_validation!(self, parent);

        if_none_else!(
            data_type,
            self,
            parent,
            parent.data_type.as_ref().map(|data_type| data_type.clone().in_scope(scope.clone()))
        );

        if let Some(data_type) = &self.data_type
            && let Some(data_type) = catalog
                .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type, source_id, errors)?
                .cloned()
        {
            if let Some(parent_data_type) = &parent.data_type {
                validate_type(&data_type, parent_data_type, catalog, errors)?;
            }
        }

        if_none_clone!(description, self, parent);

        Ok(())
    }
}

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parent: Option<(&PropertyDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((parent, scope)) = parent else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_some() {
            if parent.value.is_some() {
                // TODO: cannot override `value`
                errors.give(MissingRequiredError::new("CANNO OVERRIDE TODO1".into(), None))?;
            }
        } else if parent.value.is_some() {
            self.expression = parent.value.clone();
        } else if parent.default.is_some() {
            self.expression = parent.default.clone();
        } else if parent.required {
            errors.give(MissingRequiredError::new("property".into(), name.map(|name| name.into())))?;
        }

        complete_validation!(self, parent);

        if_none_else!(data_type, self, parent, Some(parent.type_name.clone().in_scope(scope.clone())));

        if let Some(data_type) = &self.data_type
            && let Some(data_type) = catalog
                .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type, source_id, errors)?
                .cloned()
        {
            validate_type(&data_type, &parent.type_name, catalog, errors)?;
        }

        if_none_clone!(description, self, parent);

        Ok(())
    }
}

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<(&AttributeDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((parent, scope)) = parent else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_none() && parent.default.is_some() {
            self.expression = parent.default.clone();
        }

        complete_validation!(self, parent);

        if_none_else!(data_type, self, parent, Some(parent.type_name.clone().in_scope(scope.clone())));

        if let Some(data_type) = &self.data_type
            && let Some(data_type) = catalog
                .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type, source_id, errors)?
                .cloned()
        {
            validate_type(&data_type, &parent.type_name, catalog, errors)?;
        }

        if_none_clone!(description, self, parent);

        Ok(())
    }
}

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parent: Option<(&ParameterDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((parent, scope)) = parent else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_none() {
            if parent.value.is_some() {
                self.expression = parent.value.clone();
            } else if parent.default.is_some() {
                self.expression = parent.default.clone();
            } else if parent.required {
                errors.give(MissingRequiredError::new("parameter".into(), name.map(|name| name.into())))?;
            }
        }

        complete_validation!(self, parent);

        if_none_else!(
            data_type,
            self,
            parent,
            parent.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone()))
        );

        if let Some(data_type) = &self.data_type
            && let Some(data_type) = catalog
                .get_complete_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, data_type, source_id, errors)?
                .cloned()
        {
            if let Some(parent_data_type) = &parent.type_name {
                validate_type(&data_type, parent_data_type, catalog, errors)?;
            }
        }

        if_none_clone!(description, self, parent);

        Ok(())
    }
}

// For ArtifactAssignment and ArtifactDefinition
impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            expression: self.expression.clone(),
            validation: self.validation.clone(),
            data_type: self.data_type.as_ref().map(|data_type| data_type.clone().in_scope(scope.clone())),
            schema: self.schema.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: self.validation.clone(),
            data_type: Some(self.type_name.clone().in_scope(scope.clone())),
            schema: None,
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: self.validation.clone(),
            data_type: Some(self.type_name.clone().in_scope(scope.clone())),
            schema: None,
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: self.validation.clone(),
            data_type: self.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone())),
            schema: None,
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let expression: Option<Expression<_>> = self.resolve_with_errors(errors)?;
        Ok(expression.map(|expression| expression.into()))
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
