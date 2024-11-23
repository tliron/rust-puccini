use super::{
    super::{errors::*, index::*, name::*},
    r#type::*,
};

use kutil::std::error::*;

///
#[derive(Debug)]
pub struct DefinitionEntypeContext<'own, TypeT, CatalogT> {
    ///
    pub definition_name: &'own str,
    ///
    pub type_name: &'own FullName,
    ///
    pub type_: &'own TypeT,
    ///
    pub catalog: &'own CatalogT,
    ///
    pub index: &'own Index,
}

///
#[derive(Debug)]
pub struct DefinitionDeriveContext<'own, DefinitionT, CatalogT> {
    ///
    pub definition_name: &'own str,
    ///
    pub parent_definition: &'own DefinitionT,
    ///
    pub catalog: &'own CatalogT,
    ///
    pub index: &'own Index,
}

//
// Definition
//

/// A definition is an entity that has a type, but unlike a template is part of another type or
/// template.
pub trait Definition<TypeT, CatalogT, AnnotatedT>
where
    Self: Sized,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    /// Type name.
    fn get_type_name(&self) -> Option<&FullName>;

    /// Entype.
    ///
    /// Only the first appearance of the definition in the hierarchy will be entyped.
    /// The rest will derive from it.
    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, TypeT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Derive.
    fn derive<ErrorRecipientT>(
        &self,
        context: DefinitionDeriveContext<'_, Self, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Move to scope.
    fn to_scope(&mut self, scope: &Scope);
}
