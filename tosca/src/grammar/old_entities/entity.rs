use super::id::*;

/// Entity.
pub trait OldEntity {
    /// Get ID.
    fn get_id(&self) -> &ID;

    /// Set ID.
    fn set_id(&mut self, id: ID);
}

/// Helper macro for implementing [Entity].
#[macro_export]
macro_rules! impl_entity (
    ( $type:ident $(,)? ) => {
        impl<AnnotatedT> OldEntity for $type<AnnotatedT>
        where
            AnnotatedT: Annotated + Clone + Default,
        {
            fn get_id(&self) -> &ID {
                &self.id
            }

            fn set_id(&mut self, id: ID) {
                self.id = id;
            }
        }
    }
);

//
// EntityRef
//

/// Entity reference.
#[derive(Debug)]
pub struct OldEntityRef<'own, EntityT> {
    /// ID.
    pub id: ID,

    /// Entity.
    pub entity: &'own EntityT,
}

impl<'own, EntityT> OldEntityRef<'own, EntityT> {
    /// Constructor.
    pub fn new(id: ID, entity: &'own EntityT) -> Self {
        Self { id, entity }
    }
}
