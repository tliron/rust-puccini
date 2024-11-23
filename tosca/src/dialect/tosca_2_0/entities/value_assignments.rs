use super::{
    super::{super::super::grammar::*, catalog::OldCatalog},
    attribute_definition::*,
    parameter_definition::*,
    property_definition::*,
    value_assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, zerocopy::*},
    std::collections::*,
};

//
// ValueAssignments
//

/// Map of [ValueAssignment].
pub type ValueAssignments<AnnotatedT> = BTreeMap<ByteString, ValueAssignment<AnnotatedT>>;

//
// ValueAssignmentsUtilities
//

/// Utilities for [ValueAssignments].
pub trait ValueAssignmentsUtilities<AnnotatedT> {
    /// Complete.
    fn complete(&self, parent_value_assignments: &Self) -> Self;

    /// Complete as properties.
    ///
    /// Verifies that required properties are assigned.
    fn complete_as_properties<ErrorRecipientT>(
        &self,
        property_definitions: &PropertyDefinitions<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Complete as attributes.
    fn complete_as_attributes<ErrorRecipientT>(
        &self,
        attribute_definitions: &AttributeDefinitions<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Complete as parameters.
    ///
    /// Verifies that required parameters are assigned.
    fn complete_as_parameters<ErrorRecipientT>(
        &self,
        attribute_definitions: &ParameterDefinitionMap<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Compile to Floria as properties.
    fn compile_to_floria_as_properties<ErrorRecipientT>(
        &self,
        property_defintions: &PropertyDefinitions<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<BTreeMap<ByteString, floria::Property>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Compile to Floria as attributes.
    fn compile_to_floria_as_attributes<ErrorRecipientT>(
        &self,
        attribute_defintions: &AttributeDefinitions<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<BTreeMap<ByteString, floria::Property>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

//
// ValueAssignmentsExt
//

impl<AnnotatedT> ValueAssignmentsUtilities<AnnotatedT> for ValueAssignments<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn complete(&self, parent_value_assignments: &Self) -> Self {
        let mut complete_value_assignments = self.clone();

        for (name, parent_value_assignment) in parent_value_assignments {
            if !complete_value_assignments.contains_key(name) {
                complete_value_assignments.insert(name.clone(), parent_value_assignment.clone());
            }
        }

        complete_value_assignments
    }

    fn complete_as_properties<ErrorRecipientT>(
        &self,
        property_definitions: &PropertyDefinitions<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_value_assignments = BTreeMap::default();

        for (property_name, value_assignment) in self {
            if !property_definitions.contains_key(property_name) {
                // Note: the citation location is for the expression, not the name
                errors.give(
                    UndeclaredError::new("property".into(), property_name.to_string())
                        .with_annotations_from(value_assignment),
                )?;
            }
        }

        for (property_name, property_definition) in property_definitions {
            let value_assignment = match self.get(property_name) {
                Some(value_assignment) => value_assignment.clone(),
                None => property_definition.to_assignment(property_name, errors)?,
            };
            complete_value_assignments.insert(property_name.clone(), value_assignment);
        }

        Ok(complete_value_assignments)
    }

    fn complete_as_attributes<ErrorRecipientT>(
        &self,
        attribute_definitions: &AttributeDefinitions<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_value_assignments = BTreeMap::default();

        for (attribute_name, value_assignment) in self {
            if !attribute_definitions.contains_key(attribute_name) {
                // Note: the citation location is for the expression, not the name
                errors.give(
                    UndeclaredError::new("attribute".into(), attribute_name.to_string())
                        .with_annotations_from(value_assignment),
                )?;
            }
        }

        for (attribute_name, attribute_definition) in attribute_definitions {
            let value_assignment = match self.get(attribute_name) {
                Some(value_assignment) => value_assignment.clone(),
                None => attribute_definition.to_assignment(),
            };

            complete_value_assignments.insert(attribute_name.clone(), value_assignment);
        }

        Ok(complete_value_assignments)
    }

    fn complete_as_parameters<ErrorRecipientT>(
        &self,
        parameter_definitions: &ParameterDefinitionMap<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ValueAssignments<AnnotatedT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_value_assignments = BTreeMap::default();

        for (parameter_name, value_assignment) in self {
            if !parameter_definitions.contains_key(parameter_name) {
                // Note: the citation is for the expression, not the name
                errors.give(
                    UndeclaredError::new("parameter".into(), parameter_name.to_string())
                        .with_annotations_from(value_assignment),
                )?;
            }
        }

        for (parameter_name, parameter_definition) in parameter_definitions {
            let value_assignment = match self.get(parameter_name) {
                Some(value_assignment) => value_assignment.clone(),
                None => parameter_definition.to_assignment(parameter_name, errors)?,
            };

            complete_value_assignments.insert(parameter_name.clone(), value_assignment);
        }

        Ok(complete_value_assignments)
    }

    fn compile_to_floria_as_properties<ErrorRecipientT>(
        &self,
        property_definitions: &PropertyDefinitions<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<BTreeMap<ByteString, floria::Property>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut floria_properties = BTreeMap::default();

        for (property_name, value_assignment) in self {
            match property_definitions.get(property_name) {
                Some(property_definition) => {
                    let floria_property =
                        value_assignment.compile_to_floria_as_property(property_definition, catalog, index, errors)?;
                    floria_properties.insert(property_name.clone(), floria_property);
                }

                None => tracing::warn!("property definition not found: {}", property_name),
            }
        }

        Ok(floria_properties)
    }

    fn compile_to_floria_as_attributes<ErrorRecipientT>(
        &self,
        attribute_definitions: &AttributeDefinitions<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<BTreeMap<ByteString, floria::Property>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut floria_properties = BTreeMap::default();

        for (attribute_name, value_assignment) in self {
            match attribute_definitions.get(attribute_name) {
                Some(attribute_definition) => {
                    let floria_property = value_assignment.compile_to_floria_as_attribute(
                        attribute_definition,
                        catalog,
                        index,
                        errors,
                    )?;
                    floria_properties.insert(attribute_name.clone(), floria_property);
                }

                None => tracing::warn!("attribute definition not found: {}", attribute_name),
            }
        }

        Ok(floria_properties)
    }
}
