use super::super::{errors::*, index::*};

use kutil::std::error::*;

///
#[derive(Debug)]
pub struct AssignmentCompleteContext<'own, DefinitionT, CatalogT> {
    ///
    pub name: &'own str,
    ///
    pub definition: &'own DefinitionT,
    ///
    pub catalog: &'own CatalogT,
}

///
#[derive(Debug)]
pub struct AssignmentFromDefinitionContext<'own, DefinitionT, CatalogT> {
    ///
    pub name: &'own str,
    ///
    pub definition: &'own DefinitionT,
    ///
    pub catalog: &'own CatalogT,
}

///
#[derive(Debug)]
pub struct CompileToFloriaContext<'own, CatalogT, StoreT> {
    ///
    pub floria_prefix: &'own floria::Prefix,
    ///
    pub catalog: &'own CatalogT,
    ///
    pub index: &'own Index,
    ///
    pub store: &'own StoreT,
}

impl<'own, CatalogT, StoreT> Clone for CompileToFloriaContext<'own, CatalogT, StoreT>
where
    StoreT: Clone,
{
    fn clone(&self) -> Self {
        Self { floria_prefix: self.floria_prefix, catalog: self.catalog, index: self.index, store: self.store }
    }
}

//
// Assignment
//

/// An assignment is an entity that is part of a template and is associated with a definition
/// in the template's type.
pub trait Assignment<DefinitionT, CatalogT, AnnotatedT>
where
    Self: Sized,
{
    /// Assignment entity name.
    fn get_assignment_entity_name() -> &'static str;

    /// Complete.
    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// From definition.
    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, DefinitionT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}
