use super::{
    super::{data::*, errors::*},
    assignment::*,
    definition::*,
    r#type::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, zerocopy::*},
    std::collections::*,
};

///
#[derive(Debug)]
pub struct AssignmentsAsMapCompleteContext<'own, DefinitionT, CatalogT> {
    ///
    pub definitions: &'own BTreeMap<ByteString, DefinitionT>,
    ///
    pub catalog: &'own CatalogT,
}

///
#[derive(Debug)]
pub struct AssignmentsAsMapFromDefinitionsContext<'own, DefinitionT, CatalogT> {
    ///
    pub definitions: &'own BTreeMap<ByteString, DefinitionT>,
    ///
    pub catalog: &'own CatalogT,
}

//
// AssignmentsAsMap
//

/// Assignments as a map.
pub trait AssignmentsAsMap<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT>
where
    Self: Sized,
{
    /// Complete.
    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentsAsMapCompleteContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// From definitions.
    fn from_definitions<ErrorRecipientT>(
        context: AssignmentsAsMapFromDefinitionsContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT>
    AssignmentsAsMap<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT> for BTreeMap<ByteString, AssignmentT>
where
    AssignmentT: Annotated + Assignment<DefinitionT, CatalogT, AnnotatedT> + Clone,
    DefinitionT: Definition<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
    AnnotatedT: Annotated + Default,
{
    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentsAsMapCompleteContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_assignments = BTreeMap::default();

        for (name, assignment) in self {
            if !context.definitions.contains_key(name) {
                errors.give(
                    UndeclaredError::new(AssignmentT::get_assignment_entity_name().into(), name.to_string())
                        .with_annotations_from(assignment),
                )?;
            }
        }

        for (definition_name, definition) in context.definitions {
            let assignment = match self.get(definition_name) {
                Some(assignment) => assignment.complete(
                    AssignmentCompleteContext { name: definition_name, definition, catalog: context.catalog },
                    errors,
                )?,

                None => {
                    // let citation = citation.map(|citation| citation.with_map_key(definition_name.clone()));
                    AssignmentT::from_definition(
                        AssignmentFromDefinitionContext { name: definition_name, definition, catalog: context.catalog },
                        errors,
                    )?
                }
            };

            complete_assignments.insert(definition_name.clone(), assignment.clone());
        }

        Ok(complete_assignments)
    }

    fn from_definitions<ErrorRecipientT>(
        context: AssignmentsAsMapFromDefinitionsContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut assignments = BTreeMap::default();

        for (definition_name, definition) in context.definitions {
            let assignment = AssignmentT::from_definition(
                AssignmentFromDefinitionContext { name: definition_name, definition, catalog: context.catalog },
                errors,
            )?;
            assignments.insert(definition_name.clone(), assignment);
        }

        Ok(assignments)
    }
}

///
pub struct AssignmentsAsTagValuePairsCompleteContext<'own, DefinitionT, CatalogT> {
    ///
    pub definitions: &'own TaggedValues<ByteString, DefinitionT>,
    ///
    pub catalog: &'own CatalogT,
}

///
pub struct AssignmentsAsTagValuePairsFromDefinitionsContext<'own, DefinitionT, CatalogT> {
    ///
    pub definitions: &'own TaggedValues<ByteString, DefinitionT>,
    ///
    pub catalog: &'own CatalogT,
}

//
// AssignmentsAsTagValuePairs
//

/// Assignments as tag-value pairs.
pub trait AssignmentsAsTagValuePairs<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT>
where
    Self: Sized,
{
    /// Complete.
    fn complete_assignments<ErrorRecipientT>(
        &self,
        context: AssignmentsAsTagValuePairsCompleteContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// From definitions.
    fn from_definitions<ErrorRecipientT>(
        context: AssignmentsAsTagValuePairsFromDefinitionsContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT>
    AssignmentsAsTagValuePairs<AssignmentT, DefinitionT, TypeT, CatalogT, AnnotatedT>
    for TaggedValues<ByteString, AssignmentT>
where
    AssignmentT: Annotated + Assignment<DefinitionT, CatalogT, AnnotatedT> + Clone,
    DefinitionT: Definition<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
    AnnotatedT: Annotated + Default,
{
    fn complete_assignments<ErrorRecipientT>(
        &self,
        context: AssignmentsAsTagValuePairsCompleteContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_assignments = TaggedValues::default();

        for (name, assignment) in self {
            if !context.definitions.contains_tag(name) {
                errors.give(
                    UndeclaredError::new(AssignmentT::get_assignment_entity_name().into(), name.to_string())
                        .with_annotations_from(assignment),
                )?;
            }
        }

        for (definition_name, definition) in context.definitions {
            // TODO: get is only for one
            let assignment = match self.get_first(definition_name) {
                Some(assignment) => assignment.complete(
                    AssignmentCompleteContext { name: definition_name, definition, catalog: context.catalog },
                    errors,
                )?,
                None => {
                    // let citation = citation.map(|citation| citation.with_map_key(definition_name.clone()));
                    AssignmentT::from_definition(
                        AssignmentFromDefinitionContext { name: definition_name, definition, catalog: context.catalog },
                        errors,
                    )?
                }
            };

            complete_assignments.add(definition_name.clone(), assignment.clone());
        }

        Ok(complete_assignments)
    }

    fn from_definitions<ErrorRecipientT>(
        context: AssignmentsAsTagValuePairsFromDefinitionsContext<DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut assignments = TaggedValues::default();

        for (definition_name, definition) in context.definitions {
            // let citation = citation.map(|citation| citation.with_map_key(definition_name.clone()));
            let assignment = AssignmentT::from_definition(
                AssignmentFromDefinitionContext { name: definition_name, definition, catalog: context.catalog },
                errors,
            )?;
            assignments.add(definition_name.clone(), assignment);
        }

        Ok(assignments)
    }
}
