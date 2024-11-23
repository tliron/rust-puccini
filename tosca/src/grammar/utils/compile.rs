use super::super::{catalog::*, dialect::*, entity::*, errors::*, name::*, source::*, utils::*};

use {compris::annotate::*, kutil::std::error::*};

//
//  TypeEntityCompiler
//

/// Type entity compiler.
pub struct TypeEntityCompiler<'own, DialectT> {
    /// Directory.
    pub directory: &'own ::floria::Directory,

    /// Store.
    pub store: &'own ::floria::StoreRef<'own>,

    /// Source ID.
    pub source_id: &'own SourceID,

    /// Dialect.
    pub dialect: &'own DialectT,

    /// Catalog.
    pub catalog: &'own Catalog,
}

impl<'own, DialectT> TypeEntityCompiler<'own, DialectT> {
    /// Constructor.
    pub fn new(
        directory: &'own ::floria::Directory,
        store: &'own ::floria::StoreRef<'own>,
        source_id: &'own SourceID,
        dialect: &'own DialectT,
        catalog: &'own Catalog,
    ) -> Self {
        Self { directory, store, source_id, dialect, catalog }
    }
}

impl<'own, DialectT> TypeEntityCompiler<'own, DialectT> {
    /// Compile type entity to Floria.
    pub fn compile<EntityTypeT, ErrorRecipientT, AnnotatedT>(
        &self,
        entity_kind: EntityKind,
        full_name: &FullName,
        entity_source_id: &SourceID,
        dialect_id: DialectID,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        DialectT: Dialect,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
        EntityTypeT: 'static + TypeEntity<AnnotatedT>,
        AnnotatedT: Annotated + Clone + Default,
    {
        let entity_kind_name = self.dialect.entity_kinds().represent(entity_kind);

        tracing::debug!(
            source = self.source_id.to_string(),
            name = full_name.to_string(),
            type = entity_kind_name.to_string(),
            "compiling"
        );

        let descriptor = unwrap_or_give_and_return!(
            self.catalog.get_entity::<EntityTypeT, _>(entity_kind, &full_name, self.source_id),
            errors,
            Ok(())
        )
        .descriptor();

        let mut floria_type = ::floria::Class::new_for(self.directory.clone(), full_name.to_string().into());

        floria_type.metadata.set_tosca_entity(dialect_id, entity_kind_name.clone());
        floria_type.metadata.set_tosca_version(descriptor.version);
        floria_type.metadata.set_tosca_description(descriptor.description);
        floria_type.metadata.merge_tosca_metadata(descriptor.metadata);

        if matches!(entity_source_id, SourceID::Internal(_)) {
            floria_type.metadata.set_tosca_internal(true);
        }

        if let Some(parent) = descriptor.parent {
            floria_type.metadata.set_tosca_parent(parent);

            let parent_id =
                ::floria::ID::new_for(::floria::Kind::Class, self.directory.clone(), parent.to_string().into());

            floria_type.parent_class_ids.push(parent_id);
        }

        unwrap_or_give!(self.store.add_class(floria_type), errors);

        Ok(())
    }
}
