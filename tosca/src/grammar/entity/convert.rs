use super::super::name::*;

//
// ConvertIntoScope
//

/// Convert into an entity with a different scope.
///
/// The new entity can be of a different or the same type.
pub trait ConvertIntoScope<IntoT> {
    /// Convert into an entity with a different scope.
    fn convert_into_scope(&self, scope: &Scope) -> IntoT;
}
