use super::super::{data::*, name::*};

use kutil::std::immutable::*;

//
// TypeEntityDescriptor
//

/// Type entity descriptor.
pub struct TypeEntityDescriptor<'own, AnnotatedT> {
    /// Version.
    pub version: Option<String>,

    /// Description.
    pub description: Option<&'own ByteString>,

    /// Metadata.
    pub metadata: &'own Metadata<AnnotatedT>,

    /// Parent.
    pub parent: Option<&'own FullName>,
}

//
// TypeEntity
//

/// Type entity.
pub trait TypeEntity<AnnotatedT> {
    /// Descriptor.
    fn descriptor(&self) -> TypeEntityDescriptor<'_, AnnotatedT>;
}

/// Implement [TypeEntity].
#[macro_export]
macro_rules! impl_type_entity (
    ( $type:ident $(,)? ) => {
        impl<AnnotatedT> $crate::grammar::TypeEntity<AnnotatedT> for $type<AnnotatedT>
        where
            AnnotatedT: 'static + ::compris::annotate::Annotated + Clone + Default,
        {
            fn descriptor(&self) -> $crate::grammar::TypeEntityDescriptor<'_, AnnotatedT> {
                $crate::grammar::TypeEntityDescriptor {
                    version: self.version.as_ref().map(|version| version.to_string()),
                    description: self.description.as_ref(),
                    metadata: &self.metadata,
                    parent: self.derived_from.as_ref(),
                }
            }
        }
    }
);

#[allow(unused_imports)]
pub use impl_type_entity;
