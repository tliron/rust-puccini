use super::super::name::*;

//
// IntoScoped
//

/// Convert into an entity with a different scope.
///
/// The new entity can be of a different or the same type.
pub trait IntoScoped<IntoT> {
    /// Convert into an entity with a different scope.
    fn into_scoped(&self, scope: &Scope) -> IntoT;
}
