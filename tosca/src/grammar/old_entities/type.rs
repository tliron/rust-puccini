use super::{
    super::{data::*, errors::*, index::*, name::*, utils::*},
    id::*,
    types::*,
};

use {
    compris::annotate::*,
    kutil::std::{collections::*, error::*, zerocopy::*},
};

///
#[derive(Debug)]
pub struct TypeCompleteContext<'own, TypeT, CatalogT> {
    ///
    pub type_id: &'own ID,
    ///
    pub parent_type: Option<&'own TypeT>,
    ///
    pub catalog: &'own CatalogT,
    ///
    pub index: &'own Index,
}

//
// Type
//

/// A type is an entity that can have a parent.
pub trait Type<CatalogT, AnnotatedT>
where
    Self: AnnotatedStruct + Sized,
{
    /// Type entity name.
    fn get_type_entity_name() -> &'static str;

    /// Floria group ID prefix.
    fn get_floria_group_id_prefix() -> &'static str;

    /// Version.
    fn get_version(&self) -> Option<&Version>;

    /// Description.
    fn get_description(&self) -> Option<&ByteString>;

    /// Metadata.
    fn get_metadata(&self) -> &Metadata<AnnotatedT>;

    /// The name of the parent, if we have one.
    fn get_parent_name(&self) -> Option<&FullName>;

    /// Complete.
    fn complete<ErrorRecipientT>(
        &self,
        context: TypeCompleteContext<'_, Self, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Get the citation for the field containing the name of the parent, if we have one.
    fn get_parent_field_annotations<'own>(&'own self) -> Option<&'own Annotations> {
        self.get_field_annotations("derived_from")
    }

    /// Add the citation for the field containing the name of the parent, if we have one.
    fn with_parent_name_field_annotations<AnnotatedT2>(&self, annotated: AnnotatedT2) -> AnnotatedT2
    where
        AnnotatedT2: Annotated,
    {
        match self.get_parent_field_annotations() {
            Some(annotations) => annotated.with_annotations(annotations.clone()),
            None => annotated,
        }
    }

    /// Complete.
    fn complete_into<ErrorRecipientT>(
        type_id: ID,
        complete_types: &mut FastHashMap<ID, Self>,
        types: &Types<Self, CatalogT, AnnotatedT>,
        catalog: &CatalogT,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        // Already completed?
        if complete_types.contains_key(&type_id) {
            tracing::info!("complete_into: already completed: {}", type_id);
            return Ok(());
        }

        // Complete parent first
        let parent_type_id = match types.get_parent_id_by_id(&type_id) {
            Some(parent_type_id) => {
                Self::complete_into(parent_type_id.clone(), complete_types, types, catalog, index, errors)?;
                Some(parent_type_id.clone())
            }

            // No parent (root type)
            None => None,
        };

        tracing::info!("complete_into: {}", type_id);

        match types.get_by_id(&type_id) {
            Some(type_) => match parent_type_id {
                Some(parent_type_id) => match complete_types.get(&parent_type_id) {
                    Some(complete_parent_type) => {
                        let complete_type = type_.complete(
                            TypeCompleteContext {
                                type_id: &type_id,
                                parent_type: Some(complete_parent_type),
                                catalog,
                                index,
                            },
                            errors,
                        )?;
                        complete_types.insert(type_id.clone(), complete_type);
                    }

                    None => tracing::warn!("complete_into: parent type not found for: {}", type_id),
                },

                None => {
                    // No parent (root type)
                    let complete_type = type_.complete(
                        TypeCompleteContext { type_id: &type_id, parent_type: None, catalog, index },
                        errors,
                    )?;
                    complete_types.insert(type_id.clone(), complete_type);
                }
            },

            None => tracing::warn!("complete_into: type not found: {}", type_id),
        }

        Ok(())
    }

    /// Compile to Floria.
    fn compile<StoreT, ErrorRecipientT>(
        &self,
        type_id: &ID,
        parent_type_id: Option<&ID>,
        store: &StoreT,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        tracing::info!("compile: {}", type_id);

        let mut floria_group = floria::Group::new_with(type_id.to_group_id(Self::get_floria_group_id_prefix().into()));

        floria_group.metadata.set_tosca_entity(Self::get_type_entity_name());
        floria_group.metadata.set_tosca_parent(parent_type_id);
        floria_group.metadata.set_tosca_description(self.get_description());
        floria_group.metadata.set_tosca_version(self.get_version());
        floria_group.metadata.merge_tosca_metadata(self.get_metadata());

        let id = floria_group.id.clone();

        match store.add_group(floria_group) {
            Ok(_) => Ok(Some(id)),

            Err(error) => {
                errors.give(error)?;
                Ok(None)
            }
        }
    }
}
